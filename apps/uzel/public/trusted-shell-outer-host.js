(function trustedShellOuterHost(global) {
  "use strict";

  const OUTER_URL = "nmp-shell://localhost/trusted-shell.html";
  const MAX_SURFACES = 16;
  const MAX_REQUESTS = 4096;
  const MOUNT_TIMEOUT_MS = 15000;
  const MAX_ARTIFACT_HTML_BYTES = 8 * 1024 * 1024;
  const HASH = /^[0-9a-f]{64}$/;
  const states = new Map();
  const windows = new Map();
  let nextRequest = 0;
  let disposed = false;

  function plain(value) {
    if (!value || typeof value !== "object") return false;
    const prototype = Object.getPrototypeOf(value);
    return prototype === Object.prototype || prototype === null;
  }

  function exact(value, fields) {
    if (!plain(value)) return false;
    const actual = Object.keys(value).sort();
    const expected = fields.slice().sort();
    return actual.length === expected.length &&
      actual.every((field, index) => field === expected[index]);
  }

  function validText(value, maximumBytes, allowEmpty = false) {
    return typeof value === "string" &&
      (allowEmpty || value.length > 0) &&
      value.length <= maximumBytes &&
      new TextEncoder().encode(value).byteLength <= maximumBytes &&
      !/[\u0000-\u001f\u007f]/.test(value);
  }

  function requestId() {
    nextRequest += 1;
    if (nextRequest > MAX_REQUESTS) return null;
    return `uzel-${nextRequest}`;
  }

  function bindingMatches(binding, state) {
    const expected = state.configuration.binding;
    return plain(binding) &&
      binding.surface === expected.surface &&
      binding.session === expected.session &&
      binding.manifestAuthor === expected.manifestAuthor &&
      binding.dTag === expected.dTag &&
      binding.aggregateHash === expected.aggregateHash &&
      binding.artifactDigest === expected.artifactDigest &&
      HASH.test(binding.materializedDigest);
  }

  function boundedEnvelope(value) {
    if (!plain(value)) return null;
    try {
      const serialized = JSON.stringify(value);
      return serialized.length <= 64 * 1024 ? serialized : null;
    } catch (_) {
      return null;
    }
  }

  function post(state, message) {
    if (!state.frame.contentWindow || state.frame.contentWindow !== state.outerWindow) {
      return false;
    }
    state.outerWindow.postMessage(Object.freeze(message), "*");
    return true;
  }

  function retire(state, notifyOuter = true) {
    if (states.get(state.surfaceId) !== state) return false;
    states.delete(state.surfaceId);
    windows.delete(state.outerWindow);
    global.clearTimeout(state.timer);
    if (notifyOuter) {
      const unmountId = requestId();
      if (unmountId) post(state, {
        type: "nmp.outer.unmount",
        requestId: unmountId,
        surfaceId: state.surfaceId,
        session: state.session
      });
      const disposeId = requestId();
      if (disposeId) post(state, { type: "nmp.outer.dispose", requestId: disposeId });
    }
    state.frame.remove();
    return true;
  }

  function fail(state, detail) {
    if (!retire(state)) return;
    if (typeof state.onError === "function") state.onError(state.surfaceId, detail);
  }

  function validConfiguration(surfaceId, configuration) {
    return validText(surfaceId, 128) && plain(configuration) &&
      validText(configuration.session, 256) &&
      typeof configuration.artifactBaseURL === "string" &&
      /^nmp-artifact:\/\/[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}\/$/.test(
        configuration.artifactBaseURL
      ) &&
      typeof configuration.artifactHTML === "string" &&
      configuration.artifactHTML.length <= MAX_ARTIFACT_HTML_BYTES &&
      new TextEncoder().encode(configuration.artifactHTML).byteLength <=
        MAX_ARTIFACT_HTML_BYTES &&
      validText(configuration.title, 1024, true) &&
      Array.isArray(configuration.domains) &&
      configuration.domains.length <= 64 &&
      configuration.domains.every((domain) =>
        typeof domain === "string" && /^[a-z][a-z0-9-]{0,63}$/.test(domain)
      ) && HASH.test(configuration.manifestAuthor) &&
      validText(configuration.dTag, 256) &&
      HASH.test(configuration.aggregateHash) &&
      HASH.test(configuration.artifactDigest) &&
      (typeof configuration.onReady === "undefined" ||
        typeof configuration.onReady === "function") &&
      (typeof configuration.onError === "undefined" ||
        typeof configuration.onError === "function");
  }

  function mount(surfaceId, surface, configuration) {
    if (disposed || !surface || typeof surface.replaceChildren !== "function" ||
        !validConfiguration(surfaceId, configuration)) return false;
    const existing = states.get(surfaceId);
    if (!existing && states.size >= MAX_SURFACES) return false;
    if (existing) retire(existing);
    const frame = global.document.createElement("iframe");
    frame.setAttribute("sandbox", "allow-scripts");
    frame.setAttribute("referrerpolicy", "no-referrer");
    frame.setAttribute("title", configuration.title);
    surface.replaceChildren(frame);
    const outerWindow = frame.contentWindow;
    if (!outerWindow) {
      frame.remove();
      return false;
    }
    const state = {
      surfaceId,
      session: configuration.session,
      frame,
      outerWindow,
      configuration: Object.freeze({
        session: configuration.session,
        artifactBaseURL: configuration.artifactBaseURL,
        artifactHTML: configuration.artifactHTML,
        title: configuration.title,
        domains: Object.freeze(configuration.domains.slice()),
        binding: Object.freeze({
          manifestAuthor: configuration.manifestAuthor,
          dTag: configuration.dTag,
          aggregateHash: configuration.aggregateHash,
          artifactDigest: configuration.artifactDigest,
          surface: surfaceId,
          session: configuration.session
        })
      }),
      onReady: configuration.onReady,
      onError: configuration.onError,
      outerReady: false,
      ready: false,
      timer: null
    };
    let loads = 0;
    frame.addEventListener("load", () => {
      loads += 1;
      if (loads > 1) fail(state, "outer-navigation");
    });
    frame.addEventListener("error", () => fail(state, "outer-load-error"));
    states.set(surfaceId, state);
    windows.set(outerWindow, state);
    state.timer = global.setTimeout(() => fail(state, "outer-timeout"), MOUNT_TIMEOUT_MS);
    frame.src = OUTER_URL;
    return true;
  }

  function receive(surfaceId, envelope) {
    const state = states.get(surfaceId);
    const id = requestId();
    if (!state || !state.outerReady || !id) return false;
    return post(state, {
      type: "nmp.outer.deliver",
      requestId: id,
      surfaceId,
      session: state.session,
      envelope
    });
  }

  function unmount(surfaceId) {
    const state = states.get(surfaceId);
    return state ? retire(state) : false;
  }

  function receiveOuterMessage(event) {
    const state = windows.get(event.source);
    if (!state || states.get(state.surfaceId) !== state || !plain(event.data)) return;
    const message = event.data;
    if (exact(message, ["type", "version"]) &&
        message.type === "nmp.outer.ready" && message.version === 1) {
      state.outerReady = true;
      const id = requestId();
      if (!id || !post(state, {
        type: "nmp.outer.mount",
        requestId: id,
        surfaceId: state.surfaceId,
        configuration: state.configuration
      })) fail(state, "outer-mount-refused");
      return;
    }
    if (message.type === "nmp.outer.napplet" &&
        message.surfaceId === state.surfaceId &&
        message.session === state.session && plain(message.binding) &&
        bindingMatches(message.binding, state)) {
      const envelope = boundedEnvelope(message.envelope);
      if (envelope === null) {
        fail(state, "outer-envelope-refused");
        return;
      }
      global.document.documentElement.setAttribute(
        "data-nmp-native-envelope",
        `{"session":${JSON.stringify(state.session)},"envelope":${envelope}}`
      );
      global.document.dispatchEvent(new global.CustomEvent("nmp-native-envelope"));
      global.document.documentElement.removeAttribute("data-nmp-native-envelope");
      return;
    }
    if (message.type === "nmp.outer.surface.ready" &&
        message.surfaceId === state.surfaceId && message.session === state.session &&
        bindingMatches(message.binding, state)) {
      state.ready = true;
      global.clearTimeout(state.timer);
      if (typeof state.onReady === "function") state.onReady(state.surfaceId);
      return;
    }
    if (message.type === "nmp.outer.surface.error" &&
        message.surfaceId === state.surfaceId && message.session === state.session &&
        typeof message.error === "string") {
      fail(state, "outer-surface-error");
      return;
    }
    if (exact(message, ["scope", "type"]) &&
        message.type === "nmp.outer.rate-limited" && message.scope === "parent") {
      for (const active of Array.from(states.values())) fail(active, "outer-rate-limited");
      return;
    }
    if (typeof message.type === "string" && message.type.endsWith(".result") &&
        message.ok === false &&
        (message.surfaceId === null || message.surfaceId === state.surfaceId) &&
        (message.session === null || message.session === state.session)) {
      fail(state, "outer-refused");
    }
  }

  function dispose() {
    if (disposed) return;
    disposed = true;
    for (const state of Array.from(states.values())) retire(state);
    global.removeEventListener("message", receiveOuterMessage);
    global.removeEventListener("pagehide", dispose);
  }

  global.addEventListener("message", receiveOuterMessage);
  global.addEventListener("pagehide", dispose);
  global.NMPTrustedShellHost = Object.freeze({ mount, receive, unmount });
})(window);
