#!/usr/bin/env bash
# Regenerate the Rust SDK from the upstream RIXL OpenAPI
#brew install openapi-generator openjdk
set -e

curl -sL \
  https://raw.githubusercontent.com/rixlhq/openapi/refs/heads/main/openapi.yaml \
  -o openapi.yaml

rm -rf sdk
openapi-generator generate \
  -i openapi.yaml \
  -g rust \
  -o sdk \
  -c config.yaml

# Enable the bon builder feature by default — better DX for callers.
sed -i '' 's/^default = \["native-tls"\]$/default = ["native-tls", "bon"]/' sdk/Cargo.toml
rm -f sdk/.travis.yml

# Fix openapi-generator reqwest-trait template bug for required multipart file params.
python3 post-fix.py sdk/src/apis
