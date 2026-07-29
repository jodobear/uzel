# Pinned portable trusted shell

Files in this directory are copied unchanged from
`jodobear/nampplets@fc68bce0a4793a8618445e234bcc91d69e8b96de` under
`web/trusted-shell/`. The Rust runtime remains separately pinned to
`e539378ef735ce06651fd94b71e06f9ce757cb13` because later upstream Rust relay
policy rejects the POC's deterministic plaintext loopback fixture.

Uzel bundles these reviewed browser bytes because Cargo's Rust dependency does
not expose package resources. Uzel does not fork or reproduce their source
mapping, CSP materialization, NAP prelude, or envelope projection logic.
The bounded multi-surface host is included unchanged from the same revision.
