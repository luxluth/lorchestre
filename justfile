dev:
  pnpm tauri dev

build:
  pnpm install
  just _build-{{os()}}

build-controller:
  #!/bin/bash
  cd ctl/
  cargo build --release
  cd ..
  cp ctl/target/release/lorchestrectl src-tauri/target/release/

_build-linux:
  NO_STRIP=true pnpm tauri build -v
  just build-controller

_build-macos:
  pnpm tauri build -v
  just build-controller
