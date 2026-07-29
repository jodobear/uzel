#!/usr/bin/env bash
set -euo pipefail

check_sha256() {
  local expected=$1
  local path=$2
  local actual
  actual=$(sha256sum "$path")
  actual=${actual%% *}
  if [[ "$actual" != "$expected" ]]; then
    echo "pinned asset digest mismatch: $path" >&2
    exit 1
  fi
}

check_sha256 66d2a7ed73973e422c86119c3b5c5f1914cb15bad1bfbddecb61cc2edf1c9c17 \
  fixtures/good-morning/event.json
check_sha256 ffd35eea5c84d03cdda74c23e1bbb2c40500f503833503aa688036faa52f3808 \
  fixtures/good-morning/index.html
check_sha256 f1a648993468098398a9e46413f7ed3721e2e2fce706ed3452ba1d1f04210f4d \
  apps/uzel/public/trusted-shell/trusted-shell.js
check_sha256 55c194631e7e90901dee151bdeb3cdbd93ea1576ed07bb2546d39c2b0ab590be \
  apps/uzel/public/trusted-shell/trusted-shell-surface-host.js
check_sha256 d130c02862f8934caba77bfaca4a72cc2f94158daecfaf888830c0807e2352c4 \
  apps/uzel/public/trusted-shell/trusted-shell-policy.js
check_sha256 d4c930f66df0ae1767147598d2a05b9940a06ba8f6681a1093af36e6e35251c5 \
  apps/uzel/public/trusted-shell/trusted-shell-prelude-domains.js
check_sha256 e582973897da149a64a4f4c130e2752f417d4e48d3f07192a6be7fc8bbe14ba7 \
  fixtures/follow-list/event.json
check_sha256 3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4 \
  fixtures/follow-list/index.html
check_sha256 cf95cee268f3dd05971eaa77c8dd65468b4a0d3a28f3223e8e40d18e0320967d \
  fixtures/profile-card/event.json
check_sha256 eeb037774dcc43faf6e0e13a9cf67aae8684b34c9c52921bcbd511739c46fa63 \
  fixtures/profile-card/index.html
check_sha256 004bde7aba19313cdcc2e796ef61729feaca3cc2c08a6b3ec895620b33eccd51 \
  fixtures/hostile-egress/event.json
check_sha256 7960fef5a4eb82c0634f51b0a0d27f90fea63af15a04bf0c2b75b5ffd179f61a \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK runtime=e539378ef735ce06651fd94b71e06f9ce757cb13 trusted_shell=fc68bce0a4793a8618445e234bcc91d69e8b96de'
