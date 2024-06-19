dev:
  pnpm tauri dev

build:
  pnpm install
  just _build-{{os()}}

build-daemon:
  #!/bin/bash
  echo $(pwd)
  cd daemon/
  cargo build --release
  cd ..
  cp daemon/target/release/mud src-tauri/target/release/

_build-linux:
  NO_STRIP=true pnpm tauri build -v
  just build-daemon

_build-macos:
  pnpm tauri build -v
  just build-daemon
