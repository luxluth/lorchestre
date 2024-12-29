dev:
  pnpm tauri dev

credits:

build:
  pnpm install
  just _build-{{os()}}

todos:
  snitch list

_build-linux:
  NO_STRIP=true pnpm tauri build -v --no-bundle
  notify-send $TERM "Compilation ended" --urgency=critical

_build-macos:
  pnpm tauri build -v
