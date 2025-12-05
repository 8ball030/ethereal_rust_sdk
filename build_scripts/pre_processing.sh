#!/usr/bin/env bash
set -euo pipefail

# echo "Collecting openapi.json..."
# curl https://api.ethereal.trade/openapi.json | jq > openapi.json

echo "Collected RPC config files."
curl -X 'GET' 'https://api.ethereal.trade/v1/rpc/config' \
           -H 'accept: application/json' | jq  > data/mainnet/rpc_config.json
curl -X 'GET' 'https://api.etherealtest.net/v1/rpc/config' \
            -H 'accept: application/json' | jq  > data/testnet/rpc_config.json

echo "Collected markets files."