# Post-POC extraction

Do not start fresh after a successful POC. Extract and harden the proven seams.

```mermaid
flowchart TD
    P[jodobear/uzel POC] --> A{Acceptance passed?}
    A -- no --> F[Fix or replace only falsified subsystem]
    A -- yes --> N[Extract reusable napd code\nto kehto/napd]
    A -- yes --> U[Keep product shell/policy\nin jodobear/uzel]
    A -- yes --> L[Move portable napplets\nto jodobear/napplets]
```

## Likely `kehto/napd`

- runtime composition over `nampplets`;
- exact-build/session mechanisms;
- local client/protocol if still useful;
- storage and object abstractions that are product-neutral;
- NMP provider adapter;
- deterministic testkit and hostile projection fixtures;
- daemon binary generalized only after a second consumer exists.

Before extraction, remove Uzel defaults, colors, demo IDs, and product composition policy.

## Remains in Uzel

- Tauri + Svelte shell;
- layout and visual product behavior;
- user/developer presentation;
- product configuration and packaging;
- default composition;
- future FIPS, desktop, media and wallet integrations.

## Moves to `jodobear/napplets`

- `follow-list`;
- `profile-card`;
- `profile/open` convention schema;
- portable build/conformance fixtures.

## POC shortcuts that must not silently become platform contracts

- internal protocol version `0`;
- one shell client;
- one public read identity;
- local fixture installation only;
- one trusted WebView with iframes;
- small fixed composition;
- draft relay/identity/storage pins;
- minimal developer diagnostics;
- no OS-level network sandbox if Bubblewrap proof is deferred.

## Rewrite criteria

Refactor is expected; a fresh rewrite is justified only for a falsified subsystem, such as:

- generic `nampplets` core cannot be separated from Apple-specific code;
- exact-build/session semantics cannot support a Linux host;
- Tauri/WebKit cannot maintain the required source/trust boundary;
- NMP cannot provide the required data plane through its supported facade.

Even then, preserve unaffected fixtures, napplets, NMP tests, Nix environment, and observed facts.
