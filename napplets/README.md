# Napplets

Three independent, runtime-agnostic builds live here:

- `follow-list`: reads direct follows through NAP-IDENTITY and emits the
  queryless `napplet:profile/open` convention through NAP-INC;
- `profile-card`: validates that convention payload, then reads one latest-known
  kind `0` through NAP-OUTBOX;
- `hostile-egress`: test-only browser/native-egress attempts for the strict-CSP
  host lane.

They import published Napplet packages and the shared payload contract only.
They never import Uzel, `napd`, Tauri, or each other.
