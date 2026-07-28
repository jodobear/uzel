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
check_sha256 32cb57cd2bb1064922888e5746e42dc9a63cb4df96d440026ca65efe1f2597bb \
  apps/uzel/public/trusted-shell/trusted-shell.js
check_sha256 d130c02862f8934caba77bfaca4a72cc2f94158daecfaf888830c0807e2352c4 \
  apps/uzel/public/trusted-shell/trusted-shell-policy.js
check_sha256 d4c930f66df0ae1767147598d2a05b9940a06ba8f6681a1093af36e6e35251c5 \
  apps/uzel/public/trusted-shell/trusted-shell-prelude-domains.js
check_sha256 a320f987a59628acc9e864229170d0093235f56c21c52780ceb5105e8006f9d2 \
  fixtures/follow-list/event.json
check_sha256 3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4 \
  fixtures/follow-list/index.html
check_sha256 eb4a57446c1894a2af063b08f1db4edb9ce67780c90f5d249917810b8b65a283 \
  fixtures/profile-card/event.json
check_sha256 f294c63018b76f8fadfee70f69a8d037bc4f4bf1b5f47cda4c9bfd46f0d0a923 \
  fixtures/profile-card/index.html
check_sha256 a71de606ff17075f95baeecff1602de684540c9ba8afdc5bf63e4d8b406f9a30 \
  fixtures/hostile-egress/event.json
check_sha256 94fd9d4e5ab363b17be0a6baba4b19783fabe115bced157fc081087039f1a4a9 \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK nampplets=08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf'
