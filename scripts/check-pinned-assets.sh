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
check_sha256 f46749fd7dfe85cfb7620c12f1d1094adbf41e35ae063206e03d8696a27f9f6b \
  fixtures/follow-list/event.json
check_sha256 3ae0e253b192fff4aa36a86c0ddc48f20e86551058490b2893b52fa8d3d0edf4 \
  fixtures/follow-list/index.html
check_sha256 b4eda755c1080a87bfb710f4436490017da21ef57f42cc2cbb7b5c68e61c522d \
  fixtures/profile-card/event.json
check_sha256 a5c9880dcaa1e4793283643ed0fdc2405b9ae054475bc8c13ab157d4123a2fbf \
  fixtures/profile-card/index.html
check_sha256 f222d250a2a15a892c624c424b1942e3e520a379250c8b6a760d3442a934dbc3 \
  fixtures/hostile-egress/event.json
check_sha256 2687d474ea260f00c56c6861558cea4d10b972fa3aa39bfe1268d0375a539e06 \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK nampplets=08ddb87a975dcc44c8826e4c9c7fa7cfe7f701bf'
