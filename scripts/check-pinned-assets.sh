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
check_sha256 1f7c07ec3752ab523d7db65a6eb2b008244690656baeea81b2abb1e2e111254c \
  fixtures/follow-list/event.json
check_sha256 e7227ac1df0eb0f5533d923242c13bc819b689f1317f6db3c901e3344711aa08 \
  fixtures/follow-list/index.html
check_sha256 2092db5755a68b9ed80b9cf789ed2f3e472649cb62615ae9beaa242094bdc69c \
  fixtures/profile-card/event.json
check_sha256 9c347b659caa43982218cf4b38996e5e7f17b7bc708667ce05e4a5fcdd5b021c \
  fixtures/profile-card/index.html
check_sha256 c6183534dc7d46b33c722f9d1771c62ed2a41fc92cfaae07030c6b04608b8bb3 \
  fixtures/hostile-egress/event.json
check_sha256 749d4742bde8d42a85f0719f12248203a86ebb9f7f0ace408951f08eb8e15285 \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK runtime=d533a63d519c14470f900323958509cdea1c6479 trusted_shell=fc68bce0a4793a8618445e234bcc91d69e8b96de'
