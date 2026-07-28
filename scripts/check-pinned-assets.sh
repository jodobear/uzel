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
check_sha256 f23aa814bad4b1bcd6b00cced640e47e553e222ec532c517ab65fceaf2296202 \
  fixtures/follow-list/event.json
check_sha256 3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4 \
  fixtures/follow-list/index.html
check_sha256 6bae174ff25a6e09496cf481a4b8b6f0db36b06519b458bf11e7bef2b11e962c \
  fixtures/profile-card/event.json
check_sha256 3aeb7d4079f797c1a9743bed4cf379ea49a7e9c11a0f499b49e497242c474ae9 \
  fixtures/profile-card/index.html
check_sha256 211fff99a3f7ab3e7a2a63b4b94e6721326c988c3f127b5c433abc0db3c4940c \
  fixtures/hostile-egress/event.json
check_sha256 01f37719d33342f8a43e2f3344741cabf2793baf972191b0aea63a388a51ee85 \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK nampplets=08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf'
