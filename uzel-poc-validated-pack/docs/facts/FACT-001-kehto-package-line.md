# FACT-001 — Kehto PR 204 and the 0.29 package line

- **Claim:** Kehto PR #204 is merged and its released Napplet dependency line is usable as the Uzel fixture baseline.
- **Classification:** verified fact
- **Exact source/pin:** [`kehto/web#204`](https://github.com/kehto/web/pull/204), merge `b85db51db838866de753b275b9d34ec908785bd2`; `napplet/web` source `dd7b3a728eb9c838b7218fcec7bb7bb00e7cc88b`, release `60889f1c2476e063500c7ab6624af6abe0dbcbe5`; npm integrity values in [`../../compatibility.lock`](../../compatibility.lock).
- **Probe/command:** GitHub PR API; detached checkout; `pnpm install --frozen-lockfile`; `pnpm build`; `pnpm test:unit`; `npm exec --yes --package=@napplet/conformance-cli@0.2.16 -- napplet-conformance <dist>` for `chat` and `feed`.
- **Observed result:** PR #204 merged on 2026-07-27. The exact checkout built and passed 125/125 files and 1,574/1,574 unit tests. Both generated single-file bundles failed released conformance at `boot/no-forbidden-globals`: Vite's module-preload helper contains `fetch(o.href, i)`.
- **Decision:** The merge precondition is confirmed, but the 0.29 fixture baseline is not accepted. Do not weaken conformance or grant network; require an upstream build/conformance reconciliation and republished verified fixture.
- **Affected documents/code:** `compatibility.lock`, `docs/01-validation.md`, `docs/07-source-baseline.md`, `reports/preflight.md`, `work/03-napplets.md`.
- **Revalidate when:** Kehto fixture build configuration, `@napplet/vite-plugin`, conformance, or conformance-cli changes.
