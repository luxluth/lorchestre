dev:
  just credits
  pnpm tauri dev

credits:
  ./credits.fish

build:
  just credits
  pnpm install
  just _build-{{os()}}

todos:
  snitch list

_build-linux:
  NO_STRIP=true pnpm tauri build -v --no-bundle
  notify-send ghostty "Compilation ended" --urgency=critical --app-name ghostty

_build-macos:
  pnpm tauri build -v
