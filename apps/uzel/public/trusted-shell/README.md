# Pinned portable trusted shell

Files in this directory are copied unchanged from
`jodobear/nampplets@eefa9f9d8aa463b833b4d93723dd770f81408889` under
`web/trusted-shell/`. The reviewed embedded document has SHA-256
`a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3`.
The Rust runtime remains separately pinned to
`e2f69f325a6b45213accdacfcc125e80e0687b4c`; the portable-shell merge is not a
descendant of that Uzel compatibility line and does not replace its validated
provider-push composition.

Uzel bundles these reviewed browser bytes because Cargo's Rust dependency does
not expose package resources. Uzel owns only the exact private-scheme response
and top-to-outer lifecycle adapter. It does not fork or reproduce Nampplets'
source mapping, CSP materialization, NAP prelude, envelope projection, or
canonical napplet `srcdoc` logic.
