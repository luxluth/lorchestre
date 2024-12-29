#!/usr/bin/env fish

cargo tree --no-default-features --edges=normal --depth=1 --format="{p} ({l}) ### {r}" --prefix=none -q --manifest-path=src-tauri/Cargo.toml >src/CREDITS

for dep in $(jq '.devDependencies | to_entries[] | (.key)' package.json)
    set name (echo $dep | string replace --all "\"" "")
    set pkg_path node_modules/$name/package.json

    set license (jq '.license' $pkg_path | string replace --all "\"" "")
    set pkg_version (jq '.version' $pkg_path | string replace --all "\"" "")
    set url (jq '.homepage' $pkg_path 2>/dev/null | string replace --all "\"" "")

    if test -z "$url" -o "$url" = null
        set url (jq '.repository.url' $pkg_path 2>/dev/null | string replace --all "\"" "")
    end

    if test -z "$url" -o "$url" = null
        set url (jq '.repository' $pkg_path 2>/dev/null | string replace --all "\"" "")
    end

    echo "$name v$pkg_version ($license) ### $url" >>src/CREDITS
end

sort -o src/CREDITS src/CREDITS
sed -i '/### $/d' src/CREDITS
