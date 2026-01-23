#!/usr/bin/env bash
set -euo pipefail

# Remove lines like: cargo-features = ["edition2024"] from all Cargo.toml files

while IFS= read -r -d '' file; do
  if grep -q -E '^[[:space:]]*cargo-features[[:space:]]*=[[:space:]]*\["edition2024"\][[:space:]]*$' "$file"; then
    sed -E -i '/^[[:space:]]*cargo-features[[:space:]]*=[[:space:]]*\["edition2024"\][[:space:]]*$/d' "$file"
    echo "Updated: $file"
  fi
done < <(find . -type f -name Cargo.toml -print0)

echo "Done."
