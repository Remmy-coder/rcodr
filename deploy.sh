#!/usr/bin/env bash
# Builds the release bundle straight into docs/ for GitHub Pages.
# `trunk build` wipes the whole --dist dir first, so CNAME (which trunk
# doesn't know about) has to be preserved across the build manually.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")"

cname=$(mktemp)
cp docs/CNAME "$cname"

trunk build --release --dist docs --public-url /

cp "$cname" docs/CNAME
rm -f "$cname"

echo "Built into docs/. Review with 'git status' / 'git diff', then commit and push to deploy."
