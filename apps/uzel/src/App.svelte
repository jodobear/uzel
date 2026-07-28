<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  type SurfaceLaunch = {
    surfaceToken: string;
    artifactBaseUrl: string;
    artifactHtml: string;
    title: string;
    author: string;
    dTag: string;
    aggregateHash: string;
    domains: string[];
    unavailableDomains: string[];
  };

  type HostileResult = {
    tauriInternals: boolean;
    tauriGlobal: boolean;
    wryIpc: boolean;
    parentReadable: boolean;
    rawWebkitTransport: boolean;
  };

  let status = 'Opening exact-build runtime…';
  let launch: SurfaceLaunch | null = null;
  let shellReady = false;
  let hostile: HostileResult | null = null;
  let hostileFrame: HTMLIFrameElement;

  const hostileDocument = `<!doctype html>
<html><head>
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; script-src 'unsafe-inline'; connect-src 'none'; img-src data:; style-src 'unsafe-inline'; object-src 'none'; base-uri 'none'; form-action 'none'; worker-src 'none'">
</head><body><script>
(() => {
  let parentReadable = false;
  try { parentReadable = Boolean(parent.document); } catch (_) {}
  const result = {
    tauriInternals: typeof globalThis.__TAURI_INTERNALS__ !== 'undefined',
    tauriGlobal: typeof globalThis.__TAURI__ !== 'undefined',
    wryIpc: typeof globalThis.ipc !== 'undefined',
    parentReadable,
    rawWebkitTransport: Boolean(globalThis.webkit?.messageHandlers?.ipc)
  };
  parent.postMessage({ type: 'uzel.hostile.result', result }, '*');
  parent.postMessage({
    type: 'shell.ready', session: 'attacker-session', principal: 'attacker-principal'
  }, '*');
})();
<\/script></body></html>`;

  onMount(() => {
    const receiveRuntimeEnvelope = () => {
      const payload = document.documentElement.getAttribute('data-nmp-native-envelope');
      if (!payload) return;
      let parsed: { session?: unknown; envelope?: unknown };
      try {
        parsed = JSON.parse(payload);
      } catch {
        status = 'Trusted-shell payload was malformed';
        return;
      }
      if (typeof parsed.session !== 'string' || parsed.session !== launch?.surfaceToken) {
        status = 'Rejected unmapped surface';
        return;
      }
      void invoke<string>('forward_surface_envelope', {
        surfaceToken: parsed.session,
        envelope: JSON.stringify(parsed.envelope),
      })
        .then((response) => {
          const envelope = JSON.parse(response);
          if (envelope.type === 'shell.init') {
            shellReady = true;
            status = 'Verified fixture running';
          }
          window.__nmpTrustedShellReceive(envelope);
        })
        .catch((error) => {
          status = `Runtime refused envelope: ${String(error)}`;
        });
    };

    const receiveHostileResult = (event: MessageEvent) => {
      if (event.source !== hostileFrame?.contentWindow) return;
      if (event.data?.type === 'uzel.hostile.result') {
        hostile = event.data.result as HostileResult;
        void invoke('report_hostile_probe', { report: hostile }).catch((error) => {
          status = `Hostile probe failed: ${String(error)}`;
        });
      }
    };

    document.addEventListener('nmp-native-envelope', receiveRuntimeEnvelope);
    window.addEventListener('message', receiveHostileResult);
    hostileFrame.srcdoc = hostileDocument;

    void invoke<SurfaceLaunch>('start_fixture')
      .then((started) => {
        launch = started;
        const mounted = window.__nmpTrustedShellMount({
          session: started.surfaceToken,
          artifactBaseURL: started.artifactBaseUrl,
          artifactHTML: started.artifactHtml,
          title: started.title,
          domains: started.domains,
        });
        status = mounted ? 'Waiting for NAP-SHELL…' : 'Trusted shell refused fixture';
      })
      .catch((error) => {
        status = `Fixture launch failed: ${String(error)}`;
      });

    return () => {
      document.removeEventListener('nmp-native-envelope', receiveRuntimeEnvelope);
      window.removeEventListener('message', receiveHostileResult);
    };
  });
</script>

<svelte:head>
  <meta name="description" content="Uzel Linux napplet runtime proof of concept" />
</svelte:head>

<main>
  <header>
    <div>
      <p class="eyebrow">Linux exact-build runner</p>
      <h1>Uzel</h1>
    </div>
    <div class:ready={shellReady} class="runtime-status" data-shell-ready={shellReady}>
      <span aria-hidden="true"></span>
      {status}
    </div>
  </header>

  <section class="surface-card" aria-label="Verified napplet surface">
    <div class="surface-meta">
      <strong>{launch?.title ?? 'Pinned fixture'}</strong>
      <code>{launch?.aggregateHash.slice(0, 12) ?? 'verifying'}…</code>
    </div>
    <div id="surface" aria-live="polite">
      <p>Preparing verified napplet…</p>
    </div>
  </section>

  <aside class="proofs" aria-label="Isolation evidence">
    <div>
      <span>NAP-SHELL</span>
      <strong data-proof-shell={shellReady}>{shellReady ? 'PASS' : 'PENDING'}</strong>
    </div>
    <div>
      <span>Degraded domains</span>
      <strong>{launch?.unavailableDomains.join(', ') || 'NONE'}</strong>
    </div>
    <div>
      <span>Child native authority</span>
      <strong data-proof-native={hostile ? !hostile.tauriInternals && !hostile.tauriGlobal && !hostile.wryIpc && !hostile.parentReadable : false}>
        {hostile ? (!hostile.tauriInternals && !hostile.tauriGlobal && !hostile.wryIpc && !hostile.parentReadable ? 'DENIED' : 'FAILED') : 'PENDING'}
      </strong>
    </div>
    <div>
      <span>Raw WebKit transport</span>
      <strong>{hostile?.rawWebkitTransport ? 'VISIBLE / UNAUTHENTICATED' : 'ABSENT'}</strong>
    </div>
  </aside>

  <iframe
    bind:this={hostileFrame}
    class="hostile-probe"
    sandbox="allow-scripts"
    title="Hostile isolation probe"
  ></iframe>
</main>
