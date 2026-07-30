<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount, tick } from 'svelte';
  import {
    bindingFromEvent,
    bindingMatches,
    defaultPreferences,
    DEFAULT_KEYBINDINGS,
    KEYBINDING_ACTIONS,
    parsePreferences,
    PREFERENCES_STORAGE_KEY,
    validateKeybindings,
  } from './preferences.js';

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

  type RuntimeStatus = {
    mode: string;
    activeSurfaces: string[];
    activeIdentity: string | null;
  };

  type Diagnostics = {
    snapshotRevision: number;
    activeSessions: number;
    activeIdentity: string | null;
    relayRevision: number;
    observingRelays: boolean;
    relays: number;
    omittedRelays: number;
    uncoveredAuthors: number;
    rejectedPrivateRelays: number;
    sessionsRejectedOverCap: number;
    relayDetails: RelayDiagnostic[];
    storeDegraded: string | null;
    transportDegraded: string | null;
  };

  type RelayDiagnostic = {
    relay: string;
    access: string;
    wireSubscriptions: number;
    authorsServed: number;
    lanes: string[];
    eventsByKind: string[];
    nip11Freshness: string | null;
    nip11LastError: string | null;
  };

  type RoutedEnvelope = { surfaceToken: string; envelope: string };
  type HostileReport = {
    fetch: boolean;
    xhr: boolean;
    websocket: boolean;
    eventsource: boolean;
    image: boolean;
    worker: boolean;
    serviceWorker: boolean;
    beacon: 'queued' | 'rejected';
    media: boolean;
    iframe: boolean;
    form: boolean;
    navigation: boolean;
    popup: boolean;
    tauriInternals: boolean;
    tauriGlobal: boolean;
    wryIpc: boolean;
    parentReadable: boolean;
    rawWebkitTransport: boolean;
    rawInvokeAttempted: boolean;
    identityMutationApi: boolean;
  };
  type HostileVerdict = {
    networkDenials: number;
    sentinelAccepts: number;
    nativeCalls: number;
  };
  type Pane = 'follow' | 'profile';
  type Orientation = 'horizontal' | 'vertical';
  type LogEntry = { direction: string; surface: string; type: string };
  type KeybindingAction = keyof typeof DEFAULT_KEYBINDINGS;
  type Keybindings = Record<KeybindingAction, string>;

  const FIXTURE_IDENTITY = '79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798';
  const MAX_LOG_ENTRIES = 40;

  let status = 'Connecting to the private runtime…';
  let runtime: RuntimeStatus | null = null;
  let diagnostics: Diagnostics | null = null;
  let follow: SurfaceLaunch | null = null;
  let profile: SurfaceLaunch | null = null;
  let followSurface: HTMLElement;
  let profileSurface: HTMLElement;
  let hostileSurface: HTMLElement;
  let followPane: HTMLElement;
  let profilePane: HTMLElement;
  let workspace: HTMLElement;
  let identityInput = FIXTURE_IDENTITY;
  let identityBusy = false;
  let readyCount = 0;
  let readySurfaces = new Set<string>();
  let shellHandshakeFailed = false;
  let focused: Pane = 'follow';
  let orientation: Orientation = 'horizontal';
  let split = 42;
  let fullscreen: Pane | null = null;
  let developerMode = false;
  let drawerOpen = false;
  let settingsOpen = false;
  let showEvidence = false;
  let keybindings: Keybindings = { ...DEFAULT_KEYBINDINGS };
  let draftBindings: Keybindings = { ...DEFAULT_KEYBINDINGS };
  let draftShowEvidence = false;
  let capturingAction: KeybindingAction | null = null;
  let settingsMessage = '';
  let envelopeLog: LogEntry[] = [];
  let shellReady = false;
  let hostileProbeEnabled = false;
  let hostile: SurfaceLaunch | null = null;
  let hostileProbePassed = false;
  $: shellReady = readyCount === 2 && !shellHandshakeFailed;

  function envelopeType(envelope: unknown): string {
    if (envelope && typeof envelope === 'object' && 'type' in envelope) {
      const type = (envelope as { type?: unknown }).type;
      if (typeof type === 'string') return type.slice(0, 80);
    }
    return 'unknown';
  }

  function hostileReport(envelope: unknown): HostileReport | null {
    if (!envelope || typeof envelope !== 'object') return null;
    const candidate = envelope as { type?: unknown; version?: unknown; report?: unknown };
    if (candidate.type !== 'uzel.hostile.result' || candidate.version !== 0) return null;
    if (!candidate.report || typeof candidate.report !== 'object') return null;
    const report = candidate.report as Record<string, unknown>;
    const fields = [
      'fetch', 'xhr', 'websocket', 'eventsource', 'image', 'worker', 'serviceWorker',
      'beacon', 'media', 'iframe', 'form', 'navigation', 'popup', 'tauriInternals',
      'tauriGlobal', 'wryIpc', 'parentReadable', 'rawWebkitTransport',
      'rawInvokeAttempted', 'identityMutationApi',
    ];
    if (Object.keys(report).length !== fields.length) return null;
    if (!fields.filter((field) => field !== 'beacon').every((field) => typeof report[field] === 'boolean')) return null;
    if (report.beacon !== 'queued' && report.beacon !== 'rejected') return null;
    return report as HostileReport;
  }

  async function finishHostileProbe(surfaceToken: string, report: HostileReport) {
    // Tear down while the daemon-owned sentinel is still live. WebKit may defer a
    // queued beacon until unload; finish_hostile_probe settles and samples after it.
    window.NMPTrustedShellHost.unmount(surfaceToken);
    hostile = null;
    try {
      const verdict = await invoke<HostileVerdict>('finish_hostile_probe', {
        surfaceToken,
        report,
      });
      hostileProbePassed = verdict.networkDenials === 13
        && verdict.sentinelAccepts === 0
        && verdict.nativeCalls === 0;
      if (!hostileProbePassed) throw new Error('hostile verdict was incomplete');
    } catch (error) {
      reportRuntimeFailure('Hostile boundary failed', error);
    }
  }

  function mountHostileSurface(launch: SurfaceLaunch): boolean {
    return window.NMPTrustedShellHost.mount(launch.surfaceToken, hostileSurface, {
      session: launch.surfaceToken,
      artifactBaseURL: launch.artifactBaseUrl,
      artifactHTML: launch.artifactHtml,
      title: launch.title,
      domains: launch.domains,
      onError: (surfaceToken, detail) => {
        reportRuntimeFailure(`Hostile shell rejected for ${surfaceToken}`, detail);
      },
    });
  }

  function appendLog(direction: string, surface: string, type: string) {
    envelopeLog = [...envelopeLog, { direction, surface, type }].slice(-MAX_LOG_ENTRIES);
  }

  async function refreshDiagnostics() {
    diagnostics = await invoke<Diagnostics>('runtime_diagnostics');
  }

  async function routeEnvelope(session: string, envelope: unknown) {
    appendLog('napplet → daemon', session, envelopeType(envelope));
    const delivery = await invoke<RoutedEnvelope>('forward_surface_envelope', {
      surfaceToken: session,
      envelope: JSON.stringify(envelope),
    });
    const projected = JSON.parse(delivery.envelope) as unknown;
    const type = envelopeType(projected);
    appendLog('daemon → napplet', delivery.surfaceToken, type);
    if (!window.NMPTrustedShellHost.receive(delivery.surfaceToken, projected)) {
      throw new Error('trusted shell refused the target surface');
    }
    await refreshDiagnostics();
  }

  function acknowledgeSurface(surfaceToken: string) {
    if (surfaceToken !== profile?.surfaceToken && surfaceToken !== follow?.surfaceToken) {
      return;
    }
    if (!readySurfaces.has(surfaceToken)) {
      readySurfaces = new Set([...readySurfaces, surfaceToken]);
      readyCount = readySurfaces.size;
      void invoke('report_shell_accepted', { surfaceToken }).catch((error) => {
        failShellHandshake('Shell acceptance report failed', error);
      });
      if (readyCount === 2 && !shellHandshakeFailed) {
        status = 'Two exact builds ready through NAP-SHELL';
      }
    }
  }

  function rejectSurface(surfaceToken: string, detail: string) {
    failShellHandshake(`Shell environment rejected for ${surfaceToken}`, detail);
  }

  function mountSurface(launch: SurfaceLaunch, target: HTMLElement): boolean {
    return window.NMPTrustedShellHost.mount(launch.surfaceToken, target, {
      session: launch.surfaceToken,
      artifactBaseURL: launch.artifactBaseUrl,
      artifactHTML: launch.artifactHtml,
      title: launch.title,
      domains: launch.domains,
      onReady: acknowledgeSurface,
      onError: rejectSurface,
    });
  }

  function beginShellHandshake() {
    readySurfaces = new Set();
    readyCount = 0;
    shellHandshakeFailed = false;
    status = 'Two exact builds mounted · waiting for NAP-SHELL';
  }

  function failShellHandshake(prefix: string, error: unknown) {
    shellHandshakeFailed = true;
    status = `${prefix}: ${String(error)}`;
  }

  function reportRuntimeFailure(prefix: string, error: unknown) {
    status = `${prefix}: ${String(error)}`;
  }

  async function restartActiveSurfaces() {
    beginShellHandshake();
    const previousProfile = profile;
    const previousFollow = follow;
    try {
      if (previousProfile) {
        window.NMPTrustedShellHost.unmount(previousProfile.surfaceToken);
      }
      if (previousFollow) {
        window.NMPTrustedShellHost.unmount(previousFollow.surfaceToken);
      }
      profile = null;
      follow = null;
      if (previousProfile) {
        await invoke('stop_fixture', { surfaceToken: previousProfile.surfaceToken });
      }
      if (previousFollow) {
        await invoke('stop_fixture', { surfaceToken: previousFollow.surfaceToken });
      }
      if (previousProfile) {
        profile = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'profile-card' });
      }
      if (profile && !mountSurface(profile, profileSurface)) {
        throw new Error('profile surface refused after identity change');
      }
      if (previousFollow) {
        follow = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'follow-list' });
      }
      if (follow && !mountSurface(follow, followSurface)) {
        throw new Error('follow surface refused after identity change');
      }
    } catch (error) {
      failShellHandshake('Shell restart failed', error);
      throw error;
    }
  }

  function focusPane(next: Pane) {
    focused = next;
    const pane = next === 'follow' ? followPane : profilePane;
    pane?.focus();
  }

  function enterFocusedPane() {
    const surface = focused === 'follow' ? followSurface : profileSurface;
    surface?.querySelector<HTMLIFrameElement>('iframe')?.focus();
  }

  async function selectIdentity(publicIdentity: string) {
    identityBusy = true;
    try {
      const active = await invoke<string>('select_read_identity', { publicIdentity });
      identityInput = active;
      runtime = runtime ? { ...runtime, activeIdentity: active } : runtime;
      if (follow || profile) {
        await restartActiveSurfaces();
      } else {
        status = 'Read identity selected through NMP';
      }
      await refreshDiagnostics();
    } finally {
      identityBusy = false;
    }
  }

  async function submitIdentity(event: SubmitEvent) {
    event.preventDefault();
    if (identityBusy || !shellReady) return;
    try {
      await selectIdentity(identityInput.trim());
    } catch (error) {
      if (!shellHandshakeFailed) status = `Identity refused: ${String(error)}`;
    }
  }

  function setOrientation(next: Orientation) {
    orientation = next;
    localStorage.setItem('uzel.orientation', next);
  }

  function setSplit(next: number) {
    split = Math.max(24, Math.min(76, Math.round(next)));
    localStorage.setItem('uzel.split', String(split));
  }

  function persistShellPreferences() {
    localStorage.setItem(PREFERENCES_STORAGE_KEY, JSON.stringify({
      version: 1,
      showEvidence,
      keybindings,
    }));
  }

  function openSettings() {
    draftBindings = { ...keybindings };
    draftShowEvidence = showEvidence;
    capturingAction = null;
    settingsMessage = '';
    drawerOpen = false;
    settingsOpen = true;
  }

  function closeSettings() {
    capturingAction = null;
    settingsOpen = false;
  }

  function saveSettings() {
    if (!validateKeybindings(draftBindings)) {
      settingsMessage = 'Every action needs one unique keybinding.';
      return;
    }
    keybindings = { ...draftBindings };
    showEvidence = draftShowEvidence;
    persistShellPreferences();
    settingsMessage = 'Saved.';
  }

  function resetSettings() {
    const defaults = defaultPreferences();
    draftBindings = { ...defaults.keybindings } as Keybindings;
    draftShowEvidence = defaults.showEvidence;
    settingsMessage = 'Defaults restored. Save to apply.';
  }

  function toggleEvidence() {
    showEvidence = !showEvidence;
    persistShellPreferences();
  }

  function beginResize(event: PointerEvent) {
    event.preventDefault();
    const move = (next: PointerEvent) => {
      const bounds = workspace.getBoundingClientRect();
      const position = orientation === 'horizontal'
        ? (next.clientX - bounds.left) / bounds.width
        : (next.clientY - bounds.top) / bounds.height;
      setSplit(position * 100);
    };
    const finish = () => {
      window.removeEventListener('pointermove', move);
      window.removeEventListener('pointerup', finish);
    };
    window.addEventListener('pointermove', move);
    window.addEventListener('pointerup', finish, { once: true });
  }

  function handlePaneKeys(event: KeyboardEvent) {
    if (capturingAction !== null) {
      event.preventDefault();
      event.stopPropagation();
      if (event.key === 'Escape') {
        capturingAction = null;
        settingsMessage = 'Key change cancelled.';
        return;
      }
      const binding = bindingFromEvent(event);
      if (binding !== null) {
        draftBindings = { ...draftBindings, [capturingAction]: binding };
        capturingAction = null;
        settingsMessage = 'New key captured. Save to apply.';
      }
      return;
    }
    if (event.defaultPrevented) return;
    const editable = event.target instanceof HTMLInputElement
      || event.target instanceof HTMLTextAreaElement
      || event.target instanceof HTMLSelectElement;
    if (editable) return;
    if (bindingMatches(event, keybindings.toggleSettings)) {
      event.preventDefault();
      settingsOpen ? closeSettings() : openSettings();
      return;
    }
    if (bindingMatches(event, keybindings.toggleDeveloper)) {
      event.preventDefault();
      developerMode = true;
      settingsOpen = false;
      drawerOpen = !drawerOpen;
      return;
    }
    if (bindingMatches(event, keybindings.toggleEvidence)) {
      event.preventDefault();
      toggleEvidence();
      return;
    }
    if (settingsOpen) return;
    if (bindingMatches(event, keybindings.focusPrevious)) focusPane('follow');
    if (bindingMatches(event, keybindings.focusNext)) focusPane('profile');
    if (
      bindingMatches(event, keybindings.enterPane)
      && (event.target === followPane || event.target === profilePane)
    ) {
      event.preventDefault();
      enterFocusedPane();
    }
  }

  function handleDividerKeys(event: KeyboardEvent) {
    if (event.key === 'ArrowLeft' || event.key === 'ArrowUp') {
      event.preventDefault();
      setSplit(split - 2);
    }
    if (event.key === 'ArrowRight' || event.key === 'ArrowDown') {
      event.preventDefault();
      setSplit(split + 2);
    }
  }

  onMount(() => {
    const preferences = parsePreferences(localStorage.getItem(PREFERENCES_STORAGE_KEY));
    showEvidence = preferences.showEvidence;
    keybindings = { ...preferences.keybindings } as Keybindings;
    const savedOrientation = localStorage.getItem('uzel.orientation');
    if (savedOrientation === 'horizontal' || savedOrientation === 'vertical') {
      orientation = savedOrientation;
    }
    const storedSplit = localStorage.getItem('uzel.split');
    if (storedSplit !== null) {
      const savedSplit = Number(storedSplit);
      if (Number.isFinite(savedSplit)) setSplit(savedSplit);
    }

    const receiveRuntimeEnvelope = (event: Event) => {
      const payload = document.documentElement.getAttribute('data-nmp-native-envelope');
      if (!payload) return;
      try {
        const parsed = JSON.parse(payload) as { session?: unknown; envelope?: unknown };
        if (typeof parsed.session !== 'string') throw new Error('missing mapped session');
        const report = parsed.session === hostile?.surfaceToken
          ? hostileReport(parsed.envelope)
          : null;
        if (report) {
          void finishHostileProbe(parsed.session, report);
          event.stopPropagation();
          return;
        }
        const type = envelopeType(parsed.envelope);
        void routeEnvelope(parsed.session, parsed.envelope).catch((error) => {
          if (type === 'shell.ready') {
            failShellHandshake('Shell handshake failed', error);
          } else {
            reportRuntimeFailure('Runtime refused envelope', error);
          }
        });
      } catch (error) {
        reportRuntimeFailure('Trusted-shell payload refused', error);
      }
      event.stopPropagation();
    };

    document.addEventListener('nmp-native-envelope', receiveRuntimeEnvelope);
    const diagnosticsTimer = window.setInterval(() => {
      void refreshDiagnostics().catch(() => {});
    }, 2_000);
    void (async () => {
      try {
        runtime = await invoke<RuntimeStatus>('runtime_status');
        if (runtime.activeIdentity) {
          identityInput = runtime.activeIdentity;
        } else {
          await selectIdentity(FIXTURE_IDENTITY);
        }
        profile = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'profile-card' });
        beginShellHandshake();
        if (!mountSurface(profile, profileSurface)) throw new Error('profile surface refused');
        follow = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'follow-list' });
        if (!mountSurface(follow, followSurface)) throw new Error('follow surface refused');
        await refreshDiagnostics();
        await tick();
        await invoke('report_user_mode', {
          diagnosticsHidden: document.querySelector('.developer-drawer') === null,
          unsafeControlsAbsent: document.querySelector('[data-unsafe-fixture-control]') === null,
        });
        hostileProbeEnabled = await invoke<boolean>('hostile_probe_enabled');
        if (hostileProbeEnabled) {
          await tick();
          hostile = await invoke<SurfaceLaunch>('start_hostile_probe');
          if (!mountHostileSurface(hostile)) throw new Error('hostile surface refused');
        }
      } catch (error) {
        failShellHandshake('Composition failed', error);
      }
    })();

    return () => {
      window.clearInterval(diagnosticsTimer);
      document.removeEventListener('nmp-native-envelope', receiveRuntimeEnvelope);
      if (follow) window.NMPTrustedShellHost.unmount(follow.surfaceToken);
      if (profile) window.NMPTrustedShellHost.unmount(profile.surfaceToken);
      if (hostile) window.NMPTrustedShellHost.unmount(hostile.surfaceToken);
    };
  });
</script>

<svelte:head>
  <meta name="description" content="Uzel Linux napplet runtime proof of concept" />
</svelte:head>

<svelte:window onkeydown={handlePaneKeys} />

<main class:show-evidence={showEvidence}>
  <header>
    <div class="brand">
      <p class="eyebrow">Linux exact-build runtime</p>
      <h1>Uzel</h1>
    </div>
    <div class:ready={shellReady} class="runtime-status" data-shell-ready={shellReady}>
      <span aria-hidden="true"></span>
      {status}
    </div>
    <nav aria-label="View controls">
      <button type="button" class:active={orientation === 'horizontal'} onclick={() => setOrientation('horizontal')} title="Side by side">Side</button>
      <button type="button" class:active={orientation === 'vertical'} onclick={() => setOrientation('vertical')} title="Stacked panes">Stack</button>
      <button type="button" class:active={showEvidence} onclick={toggleEvidence}>Proof</button>
      <button type="button" class:active={settingsOpen} onclick={openSettings}>Settings</button>
      <button type="button" class:active={drawerOpen} onclick={() => { developerMode = true; settingsOpen = false; drawerOpen = !drawerOpen; }}>Debug</button>
    </nav>
  </header>

  <section class="identity-bar" aria-label="Read identity">
    <form onsubmit={submitIdentity}>
      <label for="read-identity">Public read identity</label>
      <input id="read-identity" bind:value={identityInput} spellcheck="false" autocomplete="off" />
      <button type="submit" disabled={identityBusy || !shellReady}>{identityBusy ? 'Selecting…' : shellReady ? 'Use identity' : 'Waiting for panes…'}</button>
    </form>
    <div class="source-status">
      <span>{runtime?.mode === 'live' ? 'Configured relays' : 'Fixture/cache lane'}</span>
      <strong>{diagnostics?.transportDegraded ?? diagnostics?.storeDegraded ?? 'No reported degradation'}</strong>
    </div>
  </section>

  <section
    bind:this={workspace}
    class:vertical={orientation === 'vertical'}
    class:fullscreen-follow={fullscreen === 'follow'}
    class:fullscreen-profile={fullscreen === 'profile'}
    class="workspace"
    style={`--first: ${split}fr; --second: ${100 - split}fr`}
    aria-label="Composed napplet workspace"
  >
    <article
      bind:this={followPane}
      class:focused={focused === 'follow'}
      class="pane follow-pane"
      tabindex="-1"
      aria-label="Follow napplet pane. Press Enter to interact."
    >
      <div class="pane-title">
        <div><span>01</span><strong>{follow?.title ?? 'Direct follows'}</strong></div>
        <button type="button" onclick={() => fullscreen = fullscreen === 'follow' ? null : 'follow'}>{fullscreen === 'follow' ? 'Restore' : 'Fullscreen'}</button>
      </div>
      <div bind:this={followSurface} class="surface"><p>Verifying follow-list…</p></div>
      {#if showEvidence}<footer><code>{follow?.aggregateHash.slice(0, 12) ?? 'verifying'}…</code><span>{follow?.unavailableDomains.length ? `Unavailable: ${follow.unavailableDomains.join(', ')}` : 'All requested capabilities ready'}</span></footer>{/if}
    </article>

    <button
      type="button"
      class="divider"
      aria-label={`Resize napplet panes, first pane ${split}%`}
      onkeydown={handleDividerKeys}
      onpointerdown={beginResize}
    ><span></span></button>

    <article
      bind:this={profilePane}
      class:focused={focused === 'profile'}
      class="pane profile-pane"
      tabindex="-1"
      aria-label="Profile napplet pane. Press Enter to interact."
    >
      <div class="pane-title">
        <div><span>02</span><strong>{profile?.title ?? 'Profile card'}</strong></div>
        <button type="button" onclick={() => fullscreen = fullscreen === 'profile' ? null : 'profile'}>{fullscreen === 'profile' ? 'Restore' : 'Fullscreen'}</button>
      </div>
      <div bind:this={profileSurface} class="surface"><p>Verifying profile-card…</p></div>
      {#if showEvidence}<footer><code>{profile?.aggregateHash.slice(0, 12) ?? 'verifying'}…</code><span>{profile?.unavailableDomains.length ? `Unavailable: ${profile.unavailableDomains.join(', ')}` : 'All requested capabilities ready'}</span></footer>{/if}
    </article>
  </section>

  {#if showEvidence}<section class="proof-strip" aria-label="Runtime evidence">
    <div><span>NAP-SHELL</span><strong data-proof-shell={shellReady}>{readyCount}/2 {shellHandshakeFailed ? 'FAILED' : shellReady ? 'READY' : 'WAITING'}</strong></div>
    <div><span>Sessions</span><strong>{diagnostics?.activeSessions ?? 0} EXACT</strong></div>
    <div><span>NMP</span><strong>{diagnostics?.observingRelays ? `${diagnostics.relays} RELAYS` : 'CACHE-FIRST'}</strong></div>
    <div><span>Profile route</span><strong>NAP-INC</strong></div>
  </section>{/if}

  {#if settingsOpen}
    <section class="settings-page" aria-label="Settings">
      <div class="settings-card">
        <div class="settings-heading">
          <div><p class="eyebrow">Shell preferences</p><h2>Settings</h2></div>
          <button type="button" onclick={closeSettings}>Close</button>
        </div>
        <section class="settings-section">
          <h3>Appearance</h3>
          <label class="toggle-row"><input type="checkbox" bind:checked={draftShowEvidence} /><span><strong>Runtime proof chrome</strong><small>Show exact-build hashes, capability status, and the proof strip.</small></span></label>
        </section>
        <section class="settings-section">
          <h3>Keybindings</h3>
          <p class="settings-help">Select Change, then press the new key combination. Escape cancels.</p>
          <div class="keybinding-list">
            {#each KEYBINDING_ACTIONS as action}
              <div class="keybinding-row">
                <span>{action.label}</span>
                <kbd>{draftBindings[action.id as KeybindingAction]}</kbd>
                <button type="button" class:recording={capturingAction === action.id} onclick={() => { capturingAction = action.id as KeybindingAction; settingsMessage = 'Press a key combination…'; }}>{capturingAction === action.id ? 'Listening…' : 'Change'}</button>
              </div>
            {/each}
          </div>
        </section>
        <section class="settings-section network-summary">
          <h3>Network ownership</h3>
          <p>Uzel supplies operator indexers. NMP discovers each identity's NIP-65 relays, owns subscriptions, reconnects transports, and replays live demand.</p>
          <p><strong>{diagnostics?.relays ?? 0}</strong> relay sessions currently accounted · revision {diagnostics?.relayRevision ?? 0}</p>
          {#if diagnostics?.transportDegraded || diagnostics?.storeDegraded}
            <p class="network-warning">{diagnostics.transportDegraded ?? diagnostics.storeDegraded}</p>
          {/if}
        </section>
        <div class="settings-actions">
          <span role="status">{settingsMessage}</span>
          <button type="button" onclick={resetSettings}>Reset defaults</button>
          <button type="button" class="primary" onclick={saveSettings}>Save settings</button>
        </div>
      </div>
    </section>
  {/if}

  {#if developerMode && drawerOpen}
    <aside class="developer-drawer" aria-label="Developer diagnostics">
      <div class="drawer-heading"><div><p class="eyebrow">Bounded diagnostics</p><h2>Runtime evidence</h2></div><button type="button" onclick={() => drawerOpen = false}>Close</button></div>
      <dl>
        <div><dt>Snapshot</dt><dd>{diagnostics?.snapshotRevision ?? 0}</dd></div>
        <div><dt>Relay revision</dt><dd>{diagnostics?.relayRevision ?? 0}</dd></div>
        <div><dt>Active identity</dt><dd>{diagnostics?.activeIdentity ?? 'none'}</dd></div>
        <div><dt>Omitted relays</dt><dd>{diagnostics?.omittedRelays ?? 0}</dd></div>
        <div><dt>Uncovered authors</dt><dd>{diagnostics?.uncoveredAuthors ?? 0}</dd></div>
        <div><dt>Private relays rejected</dt><dd>{diagnostics?.rejectedPrivateRelays ?? 0}</dd></div>
        <div><dt>Sessions over cap</dt><dd>{diagnostics?.sessionsRejectedOverCap ?? 0}</dd></div>
      </dl>
      <section class="relay-list" aria-label="NMP relay sessions">
        <h3>Relay sessions</h3>
        {#each diagnostics?.relayDetails ?? [] as relay}
          <article>
            <strong>{relay.relay}</strong>
            <small>{relay.access} · {relay.wireSubscriptions} wire subs · {relay.authorsServed} authors</small>
            <code>{relay.lanes.join(' · ') || 'no active lanes'}</code>
            <code>events {relay.eventsByKind.join(' · ') || 'none observed'}</code>
            {#if relay.nip11LastError}<em>{relay.nip11LastError}</em>{/if}
          </article>
        {:else}
          <p>No relay session is currently accounted.</p>
        {/each}
      </section>
      <div class="envelope-log">
        {#each envelopeLog as entry}
          <div><span>{entry.direction}</span><code>{entry.type}</code><small>{entry.surface}</small></div>
        {:else}
          <p>No envelopes observed yet.</p>
        {/each}
      </div>
    </aside>
  {/if}
  {#if hostileProbeEnabled}
    <div
      bind:this={hostileSurface}
      class="hostile-probe-surface"
      data-hostile-probe-passed={hostileProbePassed}
      aria-hidden="true"
    ></div>
  {/if}
</main>

<style>
  .hostile-probe-surface {
    position: fixed;
    inset: auto auto 0 0;
    width: 1px;
    height: 1px;
    overflow: hidden;
    opacity: 0.001;
    pointer-events: none;
  }
</style>
