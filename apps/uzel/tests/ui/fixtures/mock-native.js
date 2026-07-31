(function installUzelUiHarness(global) {
  'use strict';

  const scenario = new URL(global.location.href).searchParams.get('scenario') ?? 'ready';
  const fixtureIdentity = '79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798';
  const requestedIdentity = 'd60bdad03468f5f8c85b1b10db977e310a5aafab33750dfadb37488b02bfc8d7';
  const routedProfile = 'f'.repeat(64);
  const routedProfileEventId = '5ba938bd88e383de7d687ea310e7a1c805b9c0ba9a2b6139b36efea17a326638';
  const delayedProfileResponseMs = 4_250;
  const fixtureRecords = global.__UZEL_UI_FIXTURES__;
  const realSurfaceHost = global.NMPTrustedShellHost;
  const calls = [];
  const envelopes = [];
  const activeSurfaces = new Map();
  const mountedSurfaces = new Map();
  const pendingReviews = new Set();
  let activeIdentity = scenario === 'restart-reconciliation' ? requestedIdentity : null;
  let fixtureGeneration = 0;
  let initializationFailures = [
    'initialization-failure',
    'initialization-empty-identity',
    'initialization-identity-failure',
  ].includes(scenario) ? 1 : 0;
  let identitySelectionFailures = scenario === 'initialization-identity-failure' ? 1 : 0;
  let identitySelectionPending = false;
  let reviewGeneration = 0;
  let reviewAttempts = 0;
  let confirmationAttempts = 0;
  let loadedCleanupFailures = scenario === 'cleanup-failure' ? 1 : 0;

  if (!fixtureRecords || typeof fixtureRecords !== 'object') {
    throw new Error('renderer acceptance fixture records were not injected');
  }
  if (!realSurfaceHost || typeof realSurfaceHost.mount !== 'function') {
    throw new Error('real trusted-shell surface host is unavailable');
  }

  function surfaceLaunch(fixture) {
    fixtureGeneration += 1;
    const metadata = fixtureRecords[fixture];
    const artifactHtml = metadata?.html;
    if (typeof artifactHtml !== 'string' || !artifactHtml.startsWith('<!doctype html>')) {
      throw new Error(`renderer acceptance fixture ${fixture} is missing exact HTML bytes`);
    }
    const surfaceToken = `ui-${metadata.dTag}-${fixtureGeneration}`;
    const launch = {
      surfaceToken,
      artifactBaseUrl: `nmp-artifact://00000000-0000-4000-8000-${String(fixtureGeneration).padStart(12, '0')}/`,
      artifactHtml,
      title: metadata.title,
      author: metadata.author,
      dTag: metadata.dTag,
      aggregateHash: metadata.aggregateHash,
      domains: [...new Set(['shell', ...metadata.domains])].sort(),
      unavailableDomains: [],
    };
    activeSurfaces.set(surfaceToken, launch);
    return launch;
  }

  function diagnostics() {
    return {
      snapshotRevision: 7,
      activeSessions: activeSurfaces.size,
      activeIdentity,
      relayRevision: 4,
      observingRelays: true,
      relays: 1,
      omittedRelays: 0,
      uncoveredAuthors: 0,
      rejectedPrivateRelays: 0,
      sessionsRejectedOverCap: 0,
      relayDetails: [{
        relay: 'wss://relay.ui-acceptance.invalid',
        access: 'public',
        wireSubscriptions: 2,
        authorsServed: 1,
        lanes: ['indexer:1', 'app:1'],
        eventsByKind: ['0:1', '3:1'],
        nip11Freshness: 'mocked-current',
        nip11LastError: null,
      }],
      storeDegraded: null,
      transportDegraded: null,
    };
  }

  function review(coordinate) {
    reviewGeneration += 1;
    const token = `ui-review-${reviewGeneration}`;
    pendingReviews.add(token);
    const denied = scenario === 'naddr-denied';
    const metadata = fixtureRecords['good-morning'];
    return {
      token,
      eventId: metadata.eventId,
      coordinate,
      manifestAuthor: metadata.author,
      dTag: metadata.dTag,
      title: metadata.title,
      description: 'Deterministic renderer acceptance fixture.',
      aggregateHash: metadata.aggregateHash,
      capabilities: metadata.requiredCapabilities.map((domain) => ({ domain, required: true })),
      blobSources: ['checked-in fixture HTML'],
      provenance: ['mocked native resolution; real trusted-shell artifact mount'],
      canInstall: !denied,
      blocker: denied ? 'Policy denied this exact build for renderer acceptance.' : null,
    };
  }

  function profileFor(pubkey) {
    return {
      displayName: pubkey === requestedIdentity ? 'Requested npub profile' : 'Fixture identity profile',
      about: `Deterministic latest-known profile for ${pubkey.slice(0, 12)}.`,
      nip05: pubkey === requestedIdentity ? 'requested@ui-acceptance.invalid' : 'fixture@ui-acceptance.invalid',
    };
  }

  function profileEvent(pubkey) {
    const routed = pubkey === routedProfile;
    const requested = pubkey === requestedIdentity;
    return {
      event: {
        id: routed ? routedProfileEventId : (requested ? 'a'.repeat(64) : 'b'.repeat(64)),
        pubkey,
        kind: 0,
        created_at: 1_800_000_000,
        content: JSON.stringify({
          name: routed ? 'Routed raw name' : (requested ? 'Requested raw name' : 'Fixture raw name'),
          display_name: routed
            ? 'Routed follow profile'
            : (requested ? 'Requested npub profile' : 'Fixture identity profile'),
          about: routed
            ? 'Profile loaded through NAP-INC then NAP-OUTBOX.'
            : `Deterministic latest-known profile for ${pubkey.slice(0, 12)}.`,
          picture: routed ? 'https://avatar.ui-acceptance.invalid/routed.png' : undefined,
          banner: routed ? 'https://banner.ui-acceptance.invalid/routed.png' : undefined,
          website: 'https://profile.ui-acceptance.invalid',
          lud16: 'routed@payments.ui-acceptance.invalid',
          nip05: routed ? 'routed@ui-acceptance.invalid' : 'fixture@ui-acceptance.invalid',
          bot: false,
          custom: { nested: ['complete', 0, true, null] },
          hostile_text: '<img src=x onerror="window.__escapedKind0=true">',
        }),
        tags: [],
        sig: '0'.repeat(128),
      },
    };
  }

  function isRoutedProfileQuery(envelope) {
    return envelope.type === 'outbox.query' && envelope.filters?.some((filter) => (
      filter?.kinds?.includes(0) && filter?.authors?.includes(routedProfile)
    ));
  }

  function profileEventsForQuery(envelope) {
    const known = new Set([fixtureIdentity, requestedIdentity, routedProfile]);
    const authors = new Set((envelope.filters ?? []).flatMap((filter) => (
      filter?.kinds?.includes(0) && Array.isArray(filter.authors) ? filter.authors : []
    )));
    return [...authors].filter((author) => known.has(author)).map(profileEvent);
  }

  function nativeEnvelope(surfaceToken, envelope) {
    const launch = activeSurfaces.get(surfaceToken);
    if (!launch) throw new Error(`mock received an envelope for inactive surface ${surfaceToken}`);
    switch (envelope.type) {
      case 'shell.ready':
        return {
          surfaceToken,
          envelope: {
            type: 'shell.init',
            capabilities: { domains: [...new Set(['shell', ...launch.domains])].sort() },
            services: [],
          },
        };
      case 'identity.getPublicKey':
        return { surfaceToken, envelope: { type: 'identity.getPublicKey.result', id: envelope.id, pubkey: activeIdentity } };
      case 'identity.getProfile':
        return { surfaceToken, envelope: { type: 'identity.getProfile.result', id: envelope.id, profile: profileFor(activeIdentity) } };
      case 'identity.getFollows':
        return { surfaceToken, envelope: { type: 'identity.getFollows.result', id: envelope.id, pubkeys: [routedProfile] } };
      case 'identity.getRelays':
        return { surfaceToken, envelope: { type: 'identity.getRelays.result', id: envelope.id, relays: [] } };
      case 'inc.subscribe':
        return { surfaceToken, envelope: { type: 'inc.subscribe.result', id: envelope.id } };
      case 'inc.emit': {
        const profileSurface = [...activeSurfaces.values()].find((candidate) => candidate.dTag === 'profile-card');
        if (!profileSurface) throw new Error('mock cannot route INC without an active profile surface');
        return {
          surfaceToken: profileSurface.surfaceToken,
          envelope: {
            type: 'inc.event',
            topic: envelope.topic,
            payload: envelope.payload,
            sender: launch.dTag,
          },
        };
      }
      case 'outbox.query': {
        return {
          surfaceToken,
          envelope: {
            type: 'outbox.query.result',
            id: envelope.id,
            events: profileEventsForQuery(envelope),
            incomplete: false,
          },
        };
      }
      case 'resource.bytes':
        return {
          surfaceToken,
          envelope: {
            type: 'resource.bytes.result',
            id: envelope.id,
            blob: 'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII=',
            mime: 'image/png',
          },
        };
      case 'outbox.subscribe':
        return { surfaceToken, envelope: { type: 'outbox.subscribed', subId: envelope.subId } };
      case 'outbox.close':
        return { surfaceToken, envelope: { type: 'outbox.closed', subId: envelope.subId } };
      case 'theme.get':
        return {
          surfaceToken,
          envelope: {
            type: 'theme.get.result',
            id: envelope.id,
            theme: {
              colors: { background: '#10100e', text: '#f4eee7', primary: '#a8d46f' },
              title: 'Uzel acceptance',
            },
          },
        };
      case 'debug.console':
        return { surfaceToken, envelope: { type: 'debug.console.accepted' } };
      default:
        throw new Error(`mock native boundary has no ${String(envelope.type)} envelope route`);
    }
  }

  async function invoke(command, args = {}) {
    calls.push({ command, args: structuredClone(args) });
    switch (command) {
      case 'reconcile_runtime':
        if (initializationFailures > 0) {
          initializationFailures -= 1;
          if (scenario === 'initialization-failure') activeIdentity = fixtureIdentity;
          throw new Error('mocked private runtime unavailable');
        }
        return {
          runtime: {
            mode: 'fixture',
            activeSurfaces: [],
            pendingReviews: [],
            activeIdentity,
          },
          cleanupFailures: [],
        };
      case 'select_read_identity':
        if (typeof args.publicIdentity !== 'string' || !/^[0-9a-f]{64}$/.test(args.publicIdentity)) {
          throw new Error('mock expected a 64-character lowercase hex public identity');
        }
        if (identitySelectionFailures > 0) {
          identitySelectionFailures -= 1;
          throw new Error('mocked identity selection unavailable');
        }
        identitySelectionPending = true;
        await new Promise((resolve) => global.setTimeout(resolve, 25));
        activeIdentity = args.publicIdentity;
        identitySelectionPending = false;
        return activeIdentity;
      case 'start_fixture':
        if (args.fixture !== 'profile-card' && args.fixture !== 'follow-list') {
          throw new Error(`mock refused unknown fixture ${String(args.fixture)}`);
        }
        if (identitySelectionPending || activeIdentity === null) {
          throw new Error('mock refused surface launch before identity selection completed');
        }
        return surfaceLaunch(args.fixture);
      case 'runtime_diagnostics':
        return diagnostics();
      case 'report_user_mode':
      case 'report_shell_accepted':
        return null;
      case 'hostile_probe_enabled':
        return false;
      case 'review_napplet':
        if (args.coordinate !== fixtureRecords.testNaddr) {
          throw new Error('mock accepts only the exact decoded TEST_NADDR');
        }
        reviewAttempts += 1;
        if (scenario === 'review-ambiguous' && reviewAttempts === 1) {
          throw { kind: 'reviewAmbiguous', detail: 'mock lost the first review acknowledgement' };
        }
        return review(args.coordinate);
      case 'cancel_napplet_review':
        pendingReviews.delete(args.token);
        return null;
      case 'confirm_napplet':
        confirmationAttempts += 1;
        if (!pendingReviews.has(args.token)) throw new Error('mock received an unknown review token');
        if (args.expectedAuthor !== fixtureRecords['good-morning'].author
          || args.expectedDTag !== fixtureRecords['good-morning'].dTag) {
          throw new Error('mock received changed frozen review identity');
        }
        if (args.expectedAggregateHash !== fixtureRecords['good-morning'].aggregateHash) {
          throw new Error('mock received changed frozen aggregate');
        }
        if (JSON.stringify(args.grantedDomains) !== JSON.stringify(
          fixtureRecords['good-morning'].requiredCapabilities,
        )) {
          throw new Error('mock expected every artifact-declared required domain');
        }
        if (scenario === 'confirmation-ambiguous' && confirmationAttempts === 1) {
          throw { kind: 'confirmationAmbiguous', detail: 'mock lost the first confirmation acknowledgement' };
        }
        pendingReviews.delete(args.token);
        return surfaceLaunch('good-morning');
      case 'stop_fixture': {
        const launch = activeSurfaces.get(args.surfaceToken);
        if (launch?.dTag === 'good-morning' && loadedCleanupFailures > 0) {
          loadedCleanupFailures -= 1;
          throw new Error('mock retained loaded session after stop failure');
        }
        activeSurfaces.delete(args.surfaceToken);
        return null;
      }
      case 'forward_surface_envelope': {
        const envelope = JSON.parse(args.envelope);
        envelopes.push({ surfaceToken: args.surfaceToken, envelope: structuredClone(envelope) });
        if (scenario === 'profile-delay' && isRoutedProfileQuery(envelope)) {
          await new Promise((resolve) => global.setTimeout(resolve, delayedProfileResponseMs));
        }
        const delivery = nativeEnvelope(args.surfaceToken, envelope);
        return { surfaceToken: delivery.surfaceToken, envelope: JSON.stringify(delivery.envelope) };
      }
      default:
        throw new Error(`mock native boundary has no ${command} command`);
    }
  }

  const instrumentedSurfaceHost = Object.freeze({
    mount(surfaceId, target, configuration) {
      const mounted = realSurfaceHost.mount(surfaceId, target, configuration);
      if (mounted) mountedSurfaces.set(surfaceId, configuration.title ?? 'Napplet');
      return mounted;
    },
    receive(surfaceId, envelope) {
      return realSurfaceHost.receive(surfaceId, envelope);
    },
    unmount(surfaceId) {
      const unmounted = realSurfaceHost.unmount(surfaceId);
      if (unmounted) mountedSurfaces.delete(surfaceId);
      return unmounted;
    },
  });

  global.__TAURI_INTERNALS__ = { invoke };
  global.NMPTrustedShellHost = instrumentedSurfaceHost;
  global.__UZEL_UI_HARNESS__ = Object.freeze({
    fixtureIdentity,
    requestedIdentity,
    routedProfile,
    delayedProfileResponseMs,
    calls,
    envelopes,
    scenario,
    snapshot() {
      return {
        activeIdentity,
        activeSurfaces: [...activeSurfaces.keys()].sort(),
        mountedSurfaces: [...mountedSurfaces.keys()].sort(),
        pendingReviews: [...pendingReviews].sort(),
        reviewAttempts,
        confirmationAttempts,
      };
    },
  });

  if (scenario === 'console-fault') {
    global.queueMicrotask(() => global.console.error('UZEL_UI_DELIBERATE_FAULT'));
  }
})(window);
