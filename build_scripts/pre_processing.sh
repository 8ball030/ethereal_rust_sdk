#!/usr/bin/env bash
set -euo pipefail

echo "Collecting openapi.json..."
curl -fsS https://api.meridian.xyz/openapi.json | jq > openapi.json
curl -fsS https://archive.meridian.xyz/openapi.json | jq > archive_openapi.json

echo "Collected RPC config files."
curl -fsS -X 'GET' 'https://api.meridian.xyz/v1/rpc/config' \
           -H 'accept: application/json' | jq  > data/mainnet/rpc_config.json
curl -fsS -X 'GET' 'https://api.meridiantest.net/v1/rpc/config' \
            -H 'accept: application/json' | jq  > data/testnet/rpc_config.json

echo "Collected markets files."
