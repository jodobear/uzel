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
check_sha256 5d305cb5b59739baf4842119b376411fff7bca7b63f58467436a5c336f35e0e3 \
  fixtures/follow-list/event.json
check_sha256 3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4 \
  fixtures/follow-list/index.html
check_sha256 205f939f44f58a118c03e28ddca700b7bef7963d650e9d96f79e31697ec2f08c \
  fixtures/profile-card/event.json
check_sha256 3aeb7d4079f797c1a9743bed4cf379ea49a7e9c11a0f499b49e497242c474ae9 \
  fixtures/profile-card/index.html
check_sha256 a84efe1047405cd33b1db903497ca49a9035258bbf6ded67187cf749c6e33ac7 \
  fixtures/hostile-egress/event.json
check_sha256 9afed7b8c1391b96f4ee2aff070afbeeb19dbfcffc9db5c9a3111b7c2a8c700b \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK nampplets=08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf'
