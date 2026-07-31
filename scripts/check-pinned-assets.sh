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
check_sha256 c87aeb51b24a297270e750076e0493f75c51db383e9fc3863dae509309f30356 \
  fixtures/follow-list/event.json
check_sha256 3d8161932899d3ecbea5ce52d53ae657380f6e7782271ca5b1be8aeee08465c8 \
  fixtures/follow-list/index.html
check_sha256 78f47e156e19a7727e49823ce0e000a1e1e64f1542fb9f4e29062ece7c2b34bc \
  fixtures/profile-card/event.json
check_sha256 e6d1aa2449d814219c141a4ac7aa38aa56b46be7175d75286eee5ee170c65de6 \
  fixtures/profile-card/index.html
check_sha256 c6183534dc7d46b33c722f9d1771c62ed2a41fc92cfaae07030c6b04608b8bb3 \
  fixtures/hostile-egress/event.json
check_sha256 749d4742bde8d42a85f0719f12248203a86ebb9f7f0ace408951f08eb8e15285 \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK runtime=e2f69f325a6b45213accdacfcc125e80e0687b4c trusted_shell=fc68bce0a4793a8618445e234bcc91d69e8b96de'
