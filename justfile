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
  notify-send kitty "Compilation ended" --urgency=critical --icon kitty --app-name "kitty"

_build-macos:
  pnpm tauri build -v
