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

  type CatalogCapability = { domain: string; required: boolean };
  type NappletReview = {
    token: string;
    eventId: string;
    coordinate: string;
    manifestAuthor: string;
    dTag: string;
    title: string;
    description: string | null;
    aggregateHash: string;
    capabilities: CatalogCapability[];
    blobSources: string[];
    provenance: string[];
    canInstall: boolean;
    blocker: string | null;
  };

  type RuntimeStatus = {
    mode: string;
    activeSurfaces: string[];
    pendingReviews: string[];
    activeIdentity: string | null;
  };
  type StartupCleanupRequired = {
    kind: 'surface' | 'review';
    token: string;
    detail: string;
  };
  type RuntimeReconciliation = {
    runtime: RuntimeStatus;
    cleanupFailures: StartupCleanupRequired[];
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
  type CleanupRequired = { surfaceToken: string; detail: string };
  type ReviewAmbiguous = { coordinate: string; detail: string };
  type BaseCleanupEntry = { pane: Pane; launch: SurfaceLaunch };
  type BaseRecoveryRequired = {
    entries: BaseCleanupEntry[];
    restartPanes: Pane[];
    detail: string;
  };
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
  let loaded: SurfaceLaunch | null = null;
  let cleanupRequired: CleanupRequired | null = null;
  let orphanCleanupRequired: StartupCleanupRequired[] = [];
  let baseRecoveryRequired: BaseRecoveryRequired | null = null;
  let runtimeLocked = false;
  let loadedCleanupBusy = false;
  let baseRecoveryBusy = false;
  let followSurface: HTMLElement;
  let profileSurface: HTMLElement;
  let loadedSurface: HTMLElement;
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
  let loaderOpen = false;
  let nappletCoordinate = '';
  let nappletReview: NappletReview | null = null;
  let grantedDomains = new Set<string>();
  let catalogBusy = false;
  let catalogInstalling = false;
  let reviewAmbiguous: ReviewAmbiguous | null = null;
  let confirmationAmbiguous: string | null = null;
  let catalogMessage = '';
  let catalogRequestRevision = 0;
  let requiredCapabilitiesGranted = true;
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
  let runtimeInitializationBusy = false;
  $: shellReady = readyCount === 2 && !shellHandshakeFailed;
  $: runtimeLocked = loaded !== null
    || cleanupRequired !== null
    || orphanCleanupRequired.length > 0
    || baseRecoveryRequired !== null
    || reviewAmbiguous !== null
    || confirmationAmbiguous !== null;
  $: requiredCapabilitiesGranted = nappletReview
    ? nappletReview.capabilities.every(
        (capability) => !capability.required || grantedDomains.has(capability.domain),
      )
    : true;

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

  function mountSurface(
    launch: SurfaceLaunch,
    target: HTMLElement,
    onReady: (surfaceToken: string) => void = acknowledgeSurface,
    onError: (surfaceToken: string, detail: string) => void = rejectSurface,
  ): boolean {
    return window.NMPTrustedShellHost.mount(launch.surfaceToken, target, {
      session: launch.surfaceToken,
      artifactBaseURL: launch.artifactBaseUrl,
      artifactHTML: launch.artifactHtml,
      title: launch.title,
      domains: launch.domains,
      onReady,
      onError,
    });
  }

  function acknowledgeLoadedSurface(surfaceToken: string) {
    if (surfaceToken !== loaded?.surfaceToken) return;
    status = `${loaded.title} ready through NAP-SHELL`;
    void invoke('report_shell_accepted', { surfaceToken }).catch((error) => {
      reportRuntimeFailure('Loaded shell acceptance report failed', error);
    });
  }

  async function stopLoadedSession(
    current: SurfaceLaunch,
    successStatus: string | null,
    failurePrefix: string,
  ): Promise<boolean> {
    if (loadedCleanupBusy || loaded?.surfaceToken !== current.surfaceToken) return false;
    loadedCleanupBusy = true;
    window.NMPTrustedShellHost.unmount(current.surfaceToken);
    try {
      await invoke('stop_fixture', { surfaceToken: current.surfaceToken });
      if (loaded?.surfaceToken === current.surfaceToken) loaded = null;
      if (successStatus !== null) status = successStatus;
      return true;
    } catch (error) {
      status = `${failurePrefix}: ${String(error)}. Retry cleanup before changing identity or opening another napplet.`;
      return false;
    } finally {
      loadedCleanupBusy = false;
      await refreshDiagnostics().catch(() => {});
    }
  }

  function rejectLoadedSurface(surfaceToken: string, detail: string) {
    const current = loaded;
    if (!current || current.surfaceToken !== surfaceToken) return;
    const rejected = `Loaded shell rejected for ${surfaceToken}: ${detail}`;
    status = rejected;
    void stopLoadedSession(current, `${rejected}; session stopped`, `${rejected}; cleanup failed`);
  }

  function recoverableCleanup(error: unknown): CleanupRequired | null {
    if (!error || typeof error !== 'object') return null;
    const candidate = error as { kind?: unknown; surfaceToken?: unknown; detail?: unknown };
    if (
      candidate.kind !== 'cleanupRequired'
      || typeof candidate.surfaceToken !== 'string'
      || candidate.surfaceToken.length === 0
      || candidate.surfaceToken.length > 128
      || [...candidate.surfaceToken].some((character) => /\p{Cc}/u.test(character))
      || typeof candidate.detail !== 'string'
    ) return null;
    return { surfaceToken: candidate.surfaceToken, detail: candidate.detail };
  }

  function ambiguousConfirmation(error: unknown): string | null {
    if (!error || typeof error !== 'object') return null;
    const candidate = error as { kind?: unknown; detail?: unknown };
    return candidate.kind === 'confirmationAmbiguous' && typeof candidate.detail === 'string'
      ? candidate.detail
      : null;
  }

  function ambiguousReview(error: unknown): string | null {
    if (!error || typeof error !== 'object') return null;
    const candidate = error as { kind?: unknown; detail?: unknown };
    return candidate.kind === 'reviewAmbiguous' && typeof candidate.detail === 'string'
      ? candidate.detail
      : null;
  }

  async function retryRequiredCleanup() {
    const current = cleanupRequired;
    if (!current || loadedCleanupBusy) return;
    loadedCleanupBusy = true;
    try {
      await invoke('stop_fixture', { surfaceToken: current.surfaceToken });
      if (cleanupRequired?.surfaceToken === current.surfaceToken) cleanupRequired = null;
      status = 'Unresolved napplet session stopped; exact-build loading is ready';
    } catch (error) {
      if (cleanupRequired?.surfaceToken === current.surfaceToken) {
        cleanupRequired = { ...current, detail: String(error) };
      }
      status = `Unresolved napplet cleanup failed: ${String(error)}. Retry cleanup before changing identity or opening another napplet.`;
    } finally {
      loadedCleanupBusy = false;
      await refreshDiagnostics().catch(() => {});
    }
  }

  async function initializeRuntime() {
    if (runtimeInitializationBusy) return;
    runtimeInitializationBusy = true;
    try {
      const reconciliation = await invoke<RuntimeReconciliation>('reconcile_runtime');
      runtime = reconciliation.runtime;
      orphanCleanupRequired = reconciliation.cleanupFailures;
      if (orphanCleanupRequired.length > 0) {
        failShellHandshake(
          'Runtime cleanup required',
          orphanCleanupRequired.map((entry) => `${entry.kind} ${entry.token}: ${entry.detail}`).join('; '),
        );
        return;
      }
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
      if (profile || follow) {
        await retainBaseRecovery(['profile', 'follow'], error);
      } else {
        failShellHandshake('Composition failed', error);
      }
    } finally {
      runtimeInitializationBusy = false;
    }
  }

  async function retryOrphanCleanup() {
    if (runtimeInitializationBusy || orphanCleanupRequired.length === 0) return;
    status = 'Retrying cleanup of surfaces from the previous shell…';
    await initializeRuntime();
  }

  function openNappletLoader() {
    if (!shellReady) {
      status = 'Wait for both base panes before opening another napplet';
      return;
    }
    if (runtimeLocked) {
      status = baseRecoveryRequired
        ? 'Retry unresolved pane cleanup before loading another exact build'
        : orphanCleanupRequired.length > 0
          ? 'Retry previous-shell cleanup before loading another exact build'
          : cleanupRequired
          ? 'Retry unresolved napplet cleanup before loading another exact build'
          : reviewAmbiguous
          ? 'Retry the ambiguous review before loading another exact build'
          : confirmationAmbiguous
          ? 'Retry the ambiguous confirmation before loading another exact build'
          : 'Close the open napplet before loading another exact build';
      return;
    }
    settingsOpen = false;
    drawerOpen = false;
    loaderOpen = true;
    reviewAmbiguous = null;
    confirmationAmbiguous = null;
    catalogMessage = '';
  }

  async function cancelPendingReview(review: NappletReview, failurePrefix: string): Promise<boolean> {
    try {
      await invoke('cancel_napplet_review', { token: review.token });
      return true;
    } catch (error) {
      nappletReview = review;
      loaderOpen = true;
      catalogMessage = `${failurePrefix}: ${String(error)}. Retry cancellation before leaving this loader.`;
      return false;
    }
  }

  async function closeNappletLoader() {
    if (reviewAmbiguous || confirmationAmbiguous) {
      catalogMessage = reviewAmbiguous
        ? 'Review outcome is unknown. Retry Review to reconcile it before closing.'
        : 'Confirmation outcome is unknown. Retry Install to reconcile it before closing.';
      return;
    }
    if (catalogBusy) {
      catalogMessage = 'Wait for the current catalog operation to settle before closing.';
      return;
    }
    const review = nappletReview;
    catalogRequestRevision += 1;
    if (review) {
      catalogBusy = true;
      const cancelled = await cancelPendingReview(review, 'Review cancellation failed');
      catalogBusy = false;
      if (!cancelled) return;
    }
    loaderOpen = false;
    nappletReview = null;
    grantedDomains = new Set();
    catalogMessage = '';
  }

  async function reviewNapplet(event: SubmitEvent) {
    event.preventDefault();
    const coordinate = reviewAmbiguous?.coordinate ?? nappletCoordinate.trim();
    if (!coordinate || catalogBusy || confirmationAmbiguous) return;
    const requestRevision = ++catalogRequestRevision;
    catalogBusy = true;
    catalogMessage = 'Resolving and verifying the signed manifest…';
    const previousReview = nappletReview;
    try {
      if (
        previousReview
        && !await cancelPendingReview(previousReview, 'Previous review cancellation failed')
      ) {
        return;
      }
      nappletReview = null;
      grantedDomains = new Set();
      const review = await invoke<NappletReview>('review_napplet', { coordinate });
      if (requestRevision !== catalogRequestRevision || !loaderOpen) {
        await cancelPendingReview(review, 'Superseded review cancellation failed');
        return;
      }
      nappletReview = review;
      reviewAmbiguous = null;
      grantedDomains = new Set(
        review.capabilities
          .filter((capability) => capability.required)
          .map((capability) => capability.domain),
      );
      catalogMessage = review.canInstall
        ? 'Verified. Review exact identity and capabilities before installing.'
        : (review.blocker ?? 'This exact build cannot be installed.');
    } catch (error) {
      if (requestRevision === catalogRequestRevision) {
        const ambiguous = ambiguousReview(error);
        if (ambiguous) {
          reviewAmbiguous = { coordinate, detail: ambiguous };
          loaderOpen = true;
          catalogMessage = `Review outcome is unknown: ${ambiguous}. Retry Review before closing or changing identity.`;
        } else {
          reviewAmbiguous = null;
          catalogMessage = `Review refused: ${String(error)}`;
        }
      }
    } finally {
      catalogBusy = false;
    }
  }

  function toggleGrantedDomain(domain: string, checked: boolean) {
    if (confirmationAmbiguous) return;
    const next = new Set(grantedDomains);
    checked ? next.add(domain) : next.delete(domain);
    grantedDomains = next;
  }

  function requiredCapabilitiesApproved(review: NappletReview): boolean {
    return review.capabilities.every(
      (capability) => !capability.required || grantedDomains.has(capability.domain),
    );
  }

  async function installReviewedNapplet() {
    const review = nappletReview;
    if (
      !review
      || !review.canInstall
      || !requiredCapabilitiesApproved(review)
      || catalogBusy
      || loaded !== null
      || cleanupRequired !== null
      || orphanCleanupRequired.length > 0
      || baseRecoveryRequired !== null
    ) return;
    catalogBusy = true;
    catalogInstalling = true;
    catalogMessage = confirmationAmbiguous
      ? 'Reconciling the prior confirmation with the same operation ID…'
      : 'Installing frozen bytes and applying exact-build permissions…';
    let launch: SurfaceLaunch | null = null;
    try {
      launch = await invoke<SurfaceLaunch>('confirm_napplet', {
        token: review.token,
        expectedAuthor: review.manifestAuthor,
        expectedDTag: review.dTag,
        expectedAggregateHash: review.aggregateHash,
        grantedDomains: [...grantedDomains].sort(),
      });
      loaded = launch;
      confirmationAmbiguous = null;
      await tick();
      if (!mountSurface(launch, loadedSurface, acknowledgeLoadedSurface, rejectLoadedSurface)) {
        throw new Error('trusted shell refused the loaded napplet');
      }
      nappletReview = null;
      loaderOpen = false;
      catalogMessage = '';
      await refreshDiagnostics().catch(() => {});
    } catch (error) {
      const ambiguous = ambiguousConfirmation(error);
      if (ambiguous) {
        confirmationAmbiguous = ambiguous;
        loaderOpen = true;
        catalogMessage = `Confirmation outcome is unknown: ${ambiguous}. Retry Install before closing or changing identity.`;
      } else if (launch) {
        confirmationAmbiguous = null;
        const cleaned = await stopLoadedSession(
          launch,
          null,
          'Loaded napplet cleanup failed after install refusal',
        );
        nappletReview = null;
        loaderOpen = cleaned;
        catalogMessage = cleaned
          ? `Install refused: ${String(error)} Review the naddr again to retry.`
          : `Install refused: ${String(error)} Cleanup must succeed before retry.`;
      } else {
        confirmationAmbiguous = null;
        const recoverable = recoverableCleanup(error);
        if (recoverable) {
          cleanupRequired = recoverable;
          nappletReview = null;
          grantedDomains = new Set();
          loaderOpen = false;
          catalogMessage = '';
          status = `Install transfer failed and cleanup is unresolved: ${recoverable.detail}`;
        } else {
          catalogMessage = `Install refused: ${String(error)}`;
          loaderOpen = true;
        }
      }
    } finally {
      catalogBusy = false;
      catalogInstalling = false;
    }
  }

  async function closeLoadedNapplet() {
    const current = loaded;
    if (!current || loadedCleanupBusy) return;
    await stopLoadedSession(
      current,
      'Two exact builds ready through NAP-SHELL',
      'Loaded napplet stop failed',
    );
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

  function clearStoppedBaseSurface(entry: BaseCleanupEntry) {
    if (entry.pane === 'profile' && profile?.surfaceToken === entry.launch.surfaceToken) {
      profile = null;
    }
    if (entry.pane === 'follow' && follow?.surfaceToken === entry.launch.surfaceToken) {
      follow = null;
    }
  }

  async function stopBaseSurfaces(entries: BaseCleanupEntry[]) {
    const remaining: BaseCleanupEntry[] = [];
    const failures: string[] = [];
    for (const entry of entries) {
      try {
        await invoke('stop_fixture', { surfaceToken: entry.launch.surfaceToken });
        clearStoppedBaseSurface(entry);
      } catch (error) {
        remaining.push(entry);
        failures.push(`${entry.pane}: ${String(error)}`);
      }
    }
    return { remaining, failures };
  }

  async function launchBaseSurfaces(restartPanes: Pane[]) {
    if (restartPanes.includes('profile')) {
      profile = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'profile-card' });
      if (!mountSurface(profile, profileSurface)) {
        throw new Error('profile surface refused after identity change');
      }
    }
    if (restartPanes.includes('follow')) {
      follow = await invoke<SurfaceLaunch>('start_fixture', { fixture: 'follow-list' });
      if (!mountSurface(follow, followSurface)) {
        throw new Error('follow surface refused after identity change');
      }
    }
  }

  async function retainBaseRecovery(restartPanes: Pane[], error: unknown) {
    const entries: BaseCleanupEntry[] = [];
    if (restartPanes.includes('profile') && profile) {
      window.NMPTrustedShellHost.unmount(profile.surfaceToken);
      entries.push({ pane: 'profile', launch: profile });
    }
    if (restartPanes.includes('follow') && follow) {
      window.NMPTrustedShellHost.unmount(follow.surfaceToken);
      entries.push({ pane: 'follow', launch: follow });
    }
    const stopped = await stopBaseSurfaces(entries);
    const detail = stopped.failures.length
      ? `${String(error)}; cleanup failed: ${stopped.failures.join('; ')}`
      : String(error);
    baseRecoveryRequired = { entries: stopped.remaining, restartPanes, detail };
    failShellHandshake('Shell restart failed', detail);
  }

  async function restartActiveSurfaces() {
    beginShellHandshake();
    const entries: BaseCleanupEntry[] = [];
    if (profile) entries.push({ pane: 'profile', launch: profile });
    if (follow) entries.push({ pane: 'follow', launch: follow });
    const restartPanes = entries.map((entry) => entry.pane);
    for (const entry of entries) {
      window.NMPTrustedShellHost.unmount(entry.launch.surfaceToken);
    }
    const stopped = await stopBaseSurfaces(entries);
    if (stopped.remaining.length > 0) {
      const detail = stopped.failures.join('; ');
      baseRecoveryRequired = { entries: stopped.remaining, restartPanes, detail };
      const error = new Error(detail);
      failShellHandshake('Shell restart failed', error);
      throw error;
    }
    try {
      await launchBaseSurfaces(restartPanes);
    } catch (error) {
      await retainBaseRecovery(restartPanes, error);
      throw error;
    }
  }

  async function retryBaseRecovery() {
    const current = baseRecoveryRequired;
    if (!current || baseRecoveryBusy) return;
    baseRecoveryBusy = true;
    const stopped = await stopBaseSurfaces(current.entries);
    if (stopped.remaining.length > 0) {
      const detail = stopped.failures.join('; ');
      baseRecoveryRequired = { ...current, entries: stopped.remaining, detail };
      failShellHandshake('Pane cleanup retry failed', detail);
      baseRecoveryBusy = false;
      await refreshDiagnostics().catch(() => {});
      return;
    }
    baseRecoveryRequired = null;
    beginShellHandshake();
    try {
      await launchBaseSurfaces(current.restartPanes);
    } catch (error) {
      await retainBaseRecovery(current.restartPanes, error);
    } finally {
      baseRecoveryBusy = false;
      await refreshDiagnostics().catch(() => {});
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
    if (runtimeLocked) {
      status = baseRecoveryRequired
        ? 'Retry unresolved pane cleanup before changing read identity'
        : orphanCleanupRequired.length > 0
          ? 'Retry previous-shell cleanup before changing read identity'
          : cleanupRequired
          ? 'Retry unresolved napplet cleanup before changing read identity'
          : reviewAmbiguous
          ? 'Retry the ambiguous review before changing read identity'
          : confirmationAmbiguous
          ? 'Retry the ambiguous confirmation before changing read identity'
          : 'Close the open napplet before changing read identity';
      return;
    }
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
    if (loaderOpen) void closeNappletLoader();
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
    if (settingsOpen || loaderOpen) return;
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
    void initializeRuntime();

    return () => {
      window.clearInterval(diagnosticsTimer);
      document.removeEventListener('nmp-native-envelope', receiveRuntimeEnvelope);
      const surfaceTokens = new Set<string>();
      for (const launch of [follow, profile, loaded, hostile]) {
        if (launch) {
          window.NMPTrustedShellHost.unmount(launch.surfaceToken);
          surfaceTokens.add(launch.surfaceToken);
        }
      }
      if (cleanupRequired) surfaceTokens.add(cleanupRequired.surfaceToken);
      for (const entry of orphanCleanupRequired) {
        if (entry.kind === 'surface') surfaceTokens.add(entry.token);
      }
      for (const entry of baseRecoveryRequired?.entries ?? []) {
        surfaceTokens.add(entry.launch.surfaceToken);
      }
      for (const surfaceToken of surfaceTokens) {
        void invoke('stop_fixture', { surfaceToken }).catch(() => {});
      }
      if (nappletReview && !confirmationAmbiguous) {
        void invoke('cancel_napplet_review', { token: nappletReview.token }).catch(() => {});
      }
      for (const entry of orphanCleanupRequired) {
        if (entry.kind === 'review') {
          void invoke('cancel_napplet_review', { token: entry.token }).catch(() => {});
        }
      }
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
    <div class:ready={shellReady && !cleanupRequired && orphanCleanupRequired.length === 0 && !baseRecoveryRequired} class="runtime-status" data-shell-ready={shellReady}>
      <span aria-hidden="true"></span>
      {status}
    </div>
    <nav aria-label="View controls">
      <button type="button" class:active={orientation === 'horizontal'} onclick={() => setOrientation('horizontal')} title="Side by side">Side</button>
      <button type="button" class:active={orientation === 'vertical'} onclick={() => setOrientation('vertical')} title="Stacked panes">Stack</button>
      <button type="button" class:active={loaderOpen} disabled={!shellReady || runtimeLocked} onclick={openNappletLoader} title={!shellReady ? 'Wait for both base panes' : baseRecoveryRequired ? 'Retry unresolved pane cleanup first' : orphanCleanupRequired.length > 0 ? 'Retry previous-shell cleanup first' : cleanupRequired ? 'Retry unresolved napplet cleanup first' : reviewAmbiguous ? 'Retry the ambiguous review first' : confirmationAmbiguous ? 'Retry the ambiguous confirmation first' : loaded ? 'Close the open napplet first' : 'Open a signed napplet by naddr'}>Open napplet</button>
      <button type="button" class:active={showEvidence} onclick={toggleEvidence}>Proof</button>
      <button type="button" class:active={settingsOpen} onclick={openSettings}>Settings</button>
      <button type="button" class:active={drawerOpen} onclick={() => { developerMode = true; settingsOpen = false; drawerOpen = !drawerOpen; }}>Debug</button>
    </nav>
  </header>

  <section class="identity-bar" aria-label="Read identity">
    <form onsubmit={submitIdentity}>
      <label for="read-identity">Public read identity</label>
      <input id="read-identity" bind:value={identityInput} spellcheck="false" autocomplete="off" />
      <button type="submit" disabled={identityBusy || !shellReady || runtimeLocked}>{identityBusy ? 'Selecting…' : baseRecoveryRequired ? 'Retry pane cleanup first' : orphanCleanupRequired.length > 0 ? 'Retry previous-shell cleanup first' : cleanupRequired ? 'Retry cleanup first' : reviewAmbiguous ? 'Retry review first' : confirmationAmbiguous ? 'Retry install first' : loaded ? 'Close napplet first' : shellReady ? 'Use identity' : 'Waiting for panes…'}</button>
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
    class:hidden-by-loaded={runtimeLocked}
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

  {#if loaded}
    <section class="loaded-workspace" aria-label="Loaded napplet workspace">
      <article class="pane loaded-pane">
        <div class="pane-title">
          <div><span>OPEN</span><strong>{loaded.title}</strong></div>
          <button type="button" disabled={loadedCleanupBusy} onclick={closeLoadedNapplet}>{loadedCleanupBusy ? 'Stopping…' : 'Close napplet'}</button>
        </div>
        <div bind:this={loadedSurface} class="surface"><p>Mounting verified napplet…</p></div>
        {#if showEvidence}<footer><code>{loaded.aggregateHash.slice(0, 12)}…</code><span>{loaded.unavailableDomains.length ? `Unavailable: ${loaded.unavailableDomains.join(', ')}` : 'All requested capabilities ready'}</span></footer>{/if}
      </article>
    </section>
  {/if}

  {#if cleanupRequired}
    <section class="loaded-workspace cleanup-workspace" aria-label="Pending napplet cleanup">
      <article class="pane loaded-pane cleanup-pane">
        <div class="pane-title">
          <div><span>CLEANUP</span><strong>Unresolved runtime session</strong></div>
          <button type="button" disabled={loadedCleanupBusy} onclick={retryRequiredCleanup}>{loadedCleanupBusy ? 'Stopping…' : 'Retry cleanup'}</button>
        </div>
        <div class="surface cleanup-surface"><p>{cleanupRequired.detail}</p><code>{cleanupRequired.surfaceToken}</code></div>
      </article>
    </section>
  {/if}

  {#if orphanCleanupRequired.length > 0}
    <section class="loaded-workspace cleanup-workspace" aria-label="Previous shell cleanup">
      <article class="pane loaded-pane cleanup-pane">
        <div class="pane-title">
          <div><span>RECOVER</span><strong>Previous shell left runtime sessions</strong></div>
          <button type="button" disabled={runtimeInitializationBusy} onclick={retryOrphanCleanup}>{runtimeInitializationBusy ? 'Retrying…' : 'Retry cleanup'}</button>
        </div>
        <div class="surface cleanup-surface">
          <p>Uzel will not open panes or napplets until every previous surface and review is cleaned up.</p>
          {#each orphanCleanupRequired as entry}
            <code>{entry.kind} {entry.token}: {entry.detail}</code>
          {/each}
        </div>
      </article>
    </section>
  {/if}

  {#if baseRecoveryRequired}
    <section class="loaded-workspace cleanup-workspace" aria-label="Pending base pane cleanup">
      <article class="pane loaded-pane cleanup-pane">
        <div class="pane-title">
          <div><span>RECOVER</span><strong>Profile/follows restart paused</strong></div>
          <button type="button" disabled={baseRecoveryBusy} onclick={retryBaseRecovery}>{baseRecoveryBusy ? 'Retrying…' : 'Retry panes'}</button>
        </div>
        <div class="surface cleanup-surface">
          <p>{baseRecoveryRequired.detail}</p>
          {#if baseRecoveryRequired.entries.length > 0}
            {#each baseRecoveryRequired.entries as entry}
              <code>{entry.pane}: {entry.launch.surfaceToken}</code>
            {/each}
          {:else}
            <code>Cleanup complete · pane launch awaits retry</code>
          {/if}
        </div>
      </article>
    </section>
  {/if}

  {#if showEvidence}<section class="proof-strip" aria-label="Runtime evidence">
    <div><span>NAP-SHELL</span><strong data-proof-shell={shellReady}>{readyCount}/2 {shellHandshakeFailed ? 'FAILED' : shellReady ? 'READY' : 'WAITING'}</strong></div>
    <div><span>Sessions</span><strong>{diagnostics?.activeSessions ?? 0} EXACT</strong></div>
    <div><span>NMP</span><strong>{diagnostics?.observingRelays ? `${diagnostics.relays} RELAYS` : 'CACHE-FIRST'}</strong></div>
    <div><span>Profile route</span><strong>NAP-INC</strong></div>
  </section>{/if}

  {#if loaderOpen}
    <section class="settings-page" aria-label="Open napplet">
      <div class="settings-card catalog-card">
        <div class="settings-heading">
          <div><p class="eyebrow">Signed exact build</p><h2>Open napplet</h2></div>
          <button type="button" disabled={catalogInstalling || reviewAmbiguous !== null || confirmationAmbiguous !== null} onclick={closeNappletLoader}>Close</button>
        </div>
        <form class="catalog-form" onsubmit={reviewNapplet}>
          <label for="napplet-coordinate">Napplet naddr</label>
          <div><input id="napplet-coordinate" bind:value={nappletCoordinate} disabled={reviewAmbiguous !== null || confirmationAmbiguous !== null} placeholder="naddr1… or nostr:naddr1…" spellcheck="false" autocomplete="off" /><button type="submit" disabled={catalogBusy || confirmationAmbiguous !== null || !nappletCoordinate.trim()}>{catalogBusy && !nappletReview ? (reviewAmbiguous ? 'Reconciling…' : 'Verifying…') : reviewAmbiguous ? 'Retry review' : 'Review'}</button></div>
          <small>NMP decodes the coordinate, resolves the signed kind 35129 manifest, verifies immutable bytes, and freezes this review. Relay hints do not become routing truth.</small>
        </form>
        {#if nappletReview}
          <section class="catalog-review" aria-label="Verified napplet review">
            <div class="catalog-title"><div><p class="eyebrow">Verified manifest</p><h3>{nappletReview.title}</h3></div><strong class:blocked={!nappletReview.canInstall}>{nappletReview.canInstall ? 'INSTALLABLE' : 'BLOCKED'}</strong></div>
            {#if nappletReview.description}<p>{nappletReview.description}</p>{/if}
            <dl>
              <div><dt>Author</dt><dd><code>{nappletReview.manifestAuthor}</code></dd></div>
              <div><dt>d tag</dt><dd><code>{nappletReview.dTag}</code></dd></div>
              <div><dt>Aggregate</dt><dd><code>{nappletReview.aggregateHash}</code></dd></div>
              <div><dt>Event</dt><dd><code>{nappletReview.eventId}</code></dd></div>
            </dl>
            <section class="capability-review">
              <h3>Exact-build capabilities</h3>
              {#each nappletReview.capabilities as capability}
                <label class="toggle-row"><input type="checkbox" disabled={confirmationAmbiguous !== null} checked={grantedDomains.has(capability.domain)} onchange={(event) => toggleGrantedDomain(capability.domain, event.currentTarget.checked)} /><span><strong>{capability.domain}</strong><small>{capability.required ? 'Required by verified artifact' : 'Optional'}</small></span></label>
              {:else}
                <p>This artifact requests no capability domains.</p>
              {/each}
              {#if !requiredCapabilitiesGranted}<p class="network-warning">Approve every required capability or cancel this install.</p>{/if}
            </section>
            <details><summary>Resolution evidence</summary><p>{nappletReview.provenance.join(' · ') || 'No projected provenance rows.'}</p><p>Blob sources: {nappletReview.blobSources.join(', ') || 'none'}</p></details>
          </section>
        {/if}
        <div class="settings-actions">
          <span role="status">{catalogMessage}</span>
          {#if nappletReview}<button type="button" class="primary" disabled={catalogBusy || !nappletReview.canInstall || !requiredCapabilitiesGranted} onclick={installReviewedNapplet}>{catalogBusy ? (confirmationAmbiguous ? 'Reconciling…' : 'Installing…') : confirmationAmbiguous ? 'Retry install' : 'Install exact build'}</button>{/if}
        </div>
      </div>
    </section>
  {/if}

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
          <p>Uzel supplies bounded operator relay lanes. NMP discovers each identity's NIP-65 relays, owns subscriptions, reconnects transports, and replays live demand.</p>
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
