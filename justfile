dev:
  pnpm tauri dev

build: build-linux-x86_64

build-linux-x86_64:
  NO_STRIP=true pnpm tauri build -v -t x86_64-unknown-linux-gnu

build-linux-arm:
  NO_STRIP=true pnpm tauri build -v -t aarch64-unknown-linux-gnu
