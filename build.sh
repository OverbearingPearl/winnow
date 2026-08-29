#!/bin/bash
set -e
cd "$(dirname "$0")"

echo "Building WASM..."
cd winnow-core
wasm-pack build --target web --out-dir ../web/pkg

echo "Building frontend..."
cd ../web
npm run build

echo "Done."
