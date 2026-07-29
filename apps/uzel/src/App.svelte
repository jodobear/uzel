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
    storeDegraded: string | null;
    transportDegraded: string | null;
  };

  type RoutedEnvelope = { surfaceToken: string; envelope: string };
  type Pane = 'follow' | 'profile';
  type Orientation = 'horizontal' | 'vertical';
  type LogEntry = { direction: string; surface: string; type: string };

  const FIXTURE_IDENTITY = '79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798';
  const MAX_LOG_ENTRIES = 40;

  let status = 'Connecting to the private runtime…';
  let runtime: RuntimeStatus | null = null;
  let diagnostics: Diagnostics | null = null;
  let follow: SurfaceLaunch | null = null;
  let profile: SurfaceLaunch | null = null;
  let followSurface: HTMLElement;
  let profileSurface: HTMLElement;
  let workspace: HTMLElement;
  let identityInput = FIXTURE_IDENTITY;
  let identityBusy = false;
  let readyCount = 0;
  let readySurfaces = new Set<string>();
  let focused: Pane = 'follow';
  let orientation: Orientation = 'horizontal';
  let split = 42;
  let fullscreen: Pane | null = null;
  let developerMode = false;
  let drawerOpen = false;
  let envelopeLog: LogEntry[] = [];

  function envelopeType(envelope: unknown): string {
    if (envelope && typeof envelope === 'object' && 'type' in envelope) {
      const type = (envelope as { type?: unknown }).type;
      if (typeof type === 'string') return type.slice(0, 80);
    }
    return 'unknown';
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
    if (type === 'shell.init' && !readySurfaces.has(delivery.surfaceToken)) {
      readySurfaces = new Set([...readySurfaces, delivery.surfaceToken]);
      readyCount = readySurfaces.size;
    }
    if (!window.NMPTrustedShellHost.receive(delivery.surfaceToken, projected)) {
      throw new Error('trusted shell refused the target surface');
    }
    await refreshDiagnostics();
  }

  function mountSurface(launch: SurfaceLaunch, target: HTMLElement): boolean {
    return window.NMPTrustedShellHost.mount(launch.surfaceToken, target, {
      session: launch.surfaceToken,
      artifactBaseURL: launch.artifactBaseUrl,
      artifactHTML: launch.artifactHtml,
      title: launch.title,
      domains: launch.domains,
    });
  }

  function remountActiveSurfaces() {
    readySurfaces = new Set();
    readyCount = 0;
    if (profile && !mountSurface(profile, profileSurface)) {
      throw new Error('profile surface refused after identity change');
    }
    if (follow && !mountSurface(follow, followSurface)) {
      throw new Error('follow surface refused after identity change');
    }
  }

  function focusPane(next: Pane) {
    focused = next;
    const surface = next === 'follow' ? followSurface : profileSurface;
    surface?.querySelector<HTMLIFrameElement>('iframe')?.focus();
  }

  async function selectIdentity(publicIdentity: string) {
    identityBusy = true;
    try {
      const active = await invoke<string>('select_read_identity', { publicIdentity });
      identityInput = active;
      runtime = runtime ? { ...runtime, activeIdentity: active } : runtime;
      if (follow || profile) remountActiveSurfaces();
      status = 'Read identity selected through NMP';
      await refreshDiagnostics();
    } finally {
      identityBusy = false;
    }
  }

  async function submitIdentity(event: SubmitEvent) {
    event.preventDefault();
    try {
      await selectIdentity(identityInput.trim());
    } catch (error) {
      status = `Identity refused: ${String(error)}`;
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
    if (event.defaultPrevented || event.target instanceof HTMLInputElement) return;
    const previous = orientation === 'horizontal' ? 'ArrowLeft' : 'ArrowUp';
    const next = orientation === 'horizontal' ? 'ArrowRight' : 'ArrowDown';
    if (event.key === previous) focusPane('follow');
    if (event.key === next) focusPane('profile');
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
    const savedOrientation = localStorage.getItem('uzel.orientation');
    if (savedOrientation === 'horizontal' || savedOrientation === 'vertical') {
      orientation = savedOrientation;
    }
    const savedSplit = Number(localStorage.getItem('uzel.split'));
    if (Number.isFinite(savedSplit)) setSplit(savedSplit);

    const receiveRuntimeEnvelope = (event: Event) => {
      const payload = document.documentElement.getAttribute('data-nmp-native-envelope');
      if (!payload) return;
      try {
        const parsed = JSON.parse(payload) as { session?: unknown; envelope?: unknown };
        if (typeof parsed.session !== 'string') throw new Error('missing mapped session');
        void routeEnvelope(parsed.session, parsed.envelope).catch((error) => {
          status = `Runtime refused envelope: ${String(error)}`;
        });
      } catch (error) {
        status = `Trusted-shell payload refused: ${String(error)}`;
      }
      event.stopPropagation();
    };

    document.addEventListener('nmp-native-envelope', receiveRuntimeEnvelope);
    void (async () => {
      try {
        runtime = await invoke<RuntimeStatus>('runtime_status');
        if (runtime.activeIdentity) {
          identityInput = runtime.activeIdentity;
        } else {
          await selectIdentity(FIXTURE_IDENTITY);
        }
        profile = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'profile-card' });
        if (!mountSurface(profile, profileSurface)) throw new Error('profile surface refused');
        follow = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'follow-list' });
        if (!mountSurface(follow, followSurface)) throw new Error('follow surface refused');
        await refreshDiagnostics();
        status = 'Two exact builds mounted · waiting for NAP-SHELL';
      } catch (error) {
        status = `Composition failed: ${String(error)}`;
      }
    })();

    return () => {
      document.removeEventListener('nmp-native-envelope', receiveRuntimeEnvelope);
      if (follow) window.NMPTrustedShellHost.unmount(follow.surfaceToken);
      if (profile) window.NMPTrustedShellHost.unmount(profile.surfaceToken);
    };
  });
</script>

<svelte:head>
  <meta name="description" content="Uzel Linux napplet runtime proof of concept" />
</svelte:head>

<svelte:window onkeydown={handlePaneKeys} />

<main>
  <header>
    <div class="brand">
      <p class="eyebrow">Linux exact-build runtime</p>
      <h1>Uzel</h1>
    </div>
    <div class:ready={readyCount === 2} class="runtime-status" data-shell-ready={readyCount === 2}>
      <span aria-hidden="true"></span>
      {status}
    </div>
    <nav aria-label="View controls">
      <button type="button" class:active={orientation === 'horizontal'} onclick={() => setOrientation('horizontal')}>Side by side</button>
      <button type="button" class:active={orientation === 'vertical'} onclick={() => setOrientation('vertical')}>Stacked</button>
      <button type="button" class:active={focused === 'follow'} onclick={() => focusPane('follow')}>Follow</button>
      <button type="button" class:active={focused === 'profile'} onclick={() => focusPane('profile')}>Profile</button>
      <button type="button" class:active={developerMode} onclick={() => { developerMode = !developerMode; drawerOpen = developerMode; }}>Developer</button>
    </nav>
  </header>

  <section class="identity-bar" aria-label="Read identity">
    <form onsubmit={submitIdentity}>
      <label for="read-identity">Public read identity</label>
      <input id="read-identity" bind:value={identityInput} spellcheck="false" autocomplete="off" />
      <button type="submit" disabled={identityBusy}>{identityBusy ? 'Selecting…' : 'Use identity'}</button>
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
    <article class:focused={focused === 'follow'} class="pane follow-pane">
      <div class="pane-title">
        <div><span>01</span><strong>{follow?.title ?? 'Direct follows'}</strong></div>
        <button type="button" onclick={() => fullscreen = fullscreen === 'follow' ? null : 'follow'}>{fullscreen === 'follow' ? 'Restore' : 'Fullscreen'}</button>
      </div>
      <div bind:this={followSurface} class="surface"><p>Verifying follow-list…</p></div>
      <footer><code>{follow?.aggregateHash.slice(0, 12) ?? 'verifying'}…</code><span>{follow?.unavailableDomains.join(', ') || 'exact domains available'}</span></footer>
    </article>

    <button
      type="button"
      class="divider"
      aria-label={`Resize napplet panes, first pane ${split}%`}
      onkeydown={handleDividerKeys}
      onpointerdown={beginResize}
    ><span></span></button>

    <article class:focused={focused === 'profile'} class="pane profile-pane">
      <div class="pane-title">
        <div><span>02</span><strong>{profile?.title ?? 'Profile card'}</strong></div>
        <button type="button" onclick={() => fullscreen = fullscreen === 'profile' ? null : 'profile'}>{fullscreen === 'profile' ? 'Restore' : 'Fullscreen'}</button>
      </div>
      <div bind:this={profileSurface} class="surface"><p>Verifying profile-card…</p></div>
      <footer><code>{profile?.aggregateHash.slice(0, 12) ?? 'verifying'}…</code><span>{profile?.unavailableDomains.join(', ') || 'exact domains available'}</span></footer>
    </article>
  </section>

  <section class="proof-strip" aria-label="Runtime evidence">
    <div><span>NAP-SHELL</span><strong data-proof-shell={readyCount === 2}>{readyCount}/2 READY</strong></div>
    <div><span>Sessions</span><strong>{diagnostics?.activeSessions ?? 0} EXACT</strong></div>
    <div><span>NMP</span><strong>{diagnostics?.observingRelays ? `${diagnostics.relays} RELAYS` : 'CACHE-FIRST'}</strong></div>
    <div><span>Profile route</span><strong>NAP-INC</strong></div>
  </section>

  {#if developerMode && drawerOpen}
    <aside class="developer-drawer" aria-label="Developer diagnostics">
      <div class="drawer-heading"><div><p class="eyebrow">Bounded diagnostics</p><h2>Runtime evidence</h2></div><button type="button" onclick={() => drawerOpen = false}>Close</button></div>
      <dl>
        <div><dt>Snapshot</dt><dd>{diagnostics?.snapshotRevision ?? 0}</dd></div>
        <div><dt>Relay revision</dt><dd>{diagnostics?.relayRevision ?? 0}</dd></div>
        <div><dt>Active identity</dt><dd>{diagnostics?.activeIdentity ?? 'none'}</dd></div>
        <div><dt>Omitted relays</dt><dd>{diagnostics?.omittedRelays ?? 0}</dd></div>
      </dl>
      <div class="envelope-log">
        {#each envelopeLog as entry}
          <div><span>{entry.direction}</span><code>{entry.type}</code><small>{entry.surface}</small></div>
        {:else}
          <p>No envelopes observed yet.</p>
        {/each}
      </div>
    </aside>
  {/if}
</main>
