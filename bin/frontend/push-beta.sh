#!/bin/bash

set -e

rm -rf build
pnpm install
pnpm build-beta
npx wrangler pages deployment create ./build/ --project-name pulltherug-beta