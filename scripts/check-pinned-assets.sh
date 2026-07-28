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
check_sha256 afc6d74b3ab03b6f9ea6648261336431a8f8a3fde2c7778e8b181ea02e5fb457 \
  fixtures/follow-list/event.json
check_sha256 3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4 \
  fixtures/follow-list/index.html
check_sha256 e4491368fceef32cadfb8964d3a2f9708bfcca35b9f4141eb259eca164018eab \
  fixtures/profile-card/event.json
check_sha256 173eedae314e782ddc2497070cc0f9121d3835b5a001d7c2a0be262df74726cb \
  fixtures/profile-card/index.html
check_sha256 1798cde0b7594db5a91f6feeee0ee7d2a0b071a47d732f53955945143cfb8288 \
  fixtures/hostile-egress/event.json
check_sha256 01f37719d33342f8a43e2f3344741cabf2793baf972191b0aea63a388a51ee85 \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK nampplets=08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf'
