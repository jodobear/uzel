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
check_sha256 5d188e6a877a42a377ba7155542cb9929471e09b170caa0251e3b9e8c88dfc50 \
  apps/uzel/public/trusted-shell/trusted-shell-surface-host.js
check_sha256 d130c02862f8934caba77bfaca4a72cc2f94158daecfaf888830c0807e2352c4 \
  apps/uzel/public/trusted-shell/trusted-shell-policy.js
check_sha256 d4c930f66df0ae1767147598d2a05b9940a06ba8f6681a1093af36e6e35251c5 \
  apps/uzel/public/trusted-shell/trusted-shell-prelude-domains.js
check_sha256 4e78a8c81af2fad4622f77e4ac703dce649fb597194556810d44c4ee8f8d71c4 \
  apps/uzel/public/trusted-shell/trusted-shell-embedding-contract.js
check_sha256 53443566289d21753762df8d2f7948c0896176c6e4268859a5548ad3235f2e31 \
  apps/uzel/public/trusted-shell/trusted-shell-embedding.js
check_sha256 087218b55e44b7682b46e01ce0ea4afda1e743bb94ad116a145ee5567f993055 \
  apps/uzel/public/trusted-shell/trusted-shell.css
check_sha256 a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3 \
  apps/uzel/public/trusted-shell/trusted-shell-embedded.html
check_sha256 0250f3ebb47d9a289b44315c0ebf937403caf7835133b215ac117491b0f26d15 \
  apps/uzel/public/trusted-shell/trusted-shell-embedded.sha256
check_sha256 b145fd991e1a7c9600962351103bdb1464a2fe85af2cb644cade6db2377fcf25 \
  fixtures/follow-list/event.json
check_sha256 cb331fee5ca80e58b8cecfbaec8fc6c74960bad0f6fdb5133fd0d17203bfd204 \
  fixtures/follow-list/index.html
check_sha256 bf93d3adec14237e799bb507464c4c3175e8525db460c7f87f6be54331295980 \
  fixtures/profile-card/event.json
check_sha256 5b570417414fc9cfc81cf6124893b2a4833175693da27103c240754106d63954 \
  fixtures/profile-card/index.html
check_sha256 c6183534dc7d46b33c722f9d1771c62ed2a41fc92cfaae07030c6b04608b8bb3 \
  fixtures/hostile-egress/event.json
check_sha256 749d4742bde8d42a85f0719f12248203a86ebb9f7f0ace408951f08eb8e15285 \
  fixtures/hostile-egress/index.html

echo 'PINNED_ASSETS_OK runtime=e2f69f325a6b45213accdacfcc125e80e0687b4c trusted_shell=eefa9f9d8aa463b833b4d93723dd770f81408889 embedded_sha256=a3e6c18e8724329332bd15a039282a8a0bcf5ec93577b97752f46721df80fba3'
