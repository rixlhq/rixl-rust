#!/usr/bin/env bash
# Regenerate the Rust SDK from the upstream RIXL OpenAPI
#brew install openapi-generator openjdk
set -e

OPENAPI_GENERATOR=${OPENAPI_GENERATOR:-openapi-generator}

curl -fsSL \
  https://raw.githubusercontent.com/rixlhq/openapi/refs/heads/main/openapi.yaml \
  -o openapi.yaml

# openapi-generator's rust target can't handle these spec shapes: "not" crashes
# it, dotted path params break format!, and a oneOf with array and null variants
# generates an invalid enum. Value without oneOf becomes serde_json::Value.
# Dropping the proto package from operationIds keeps method names short.
yq -i '
  .servers = [{"url": "https://api.rixl.com"}] |
  del(.. | select(tag == "!!map") | .not) |
  .components.schemas["google.protobuf.Value"] |= del(.oneOf) |
  .paths |= with_entries(.key |= sub("\{(\w+)\.(\w+)\}"; "{${1}_${2}}")) |
  (.paths[][].parameters[]? | select(.in == "path") | .name) |= sub("\."; "_") |
  (.paths[][].operationId | select(. != null)) |= sub("^.*\."; "")
' openapi.yaml

rm -rf sdk
$OPENAPI_GENERATOR generate \
  -i openapi.yaml \
  -g rust \
  -o sdk \
  -c config.yaml \
  -t templates \
  --skip-validate-spec

# Enable the bon builder feature by default — better DX for callers.
sed -i.bak 's/^default = \["native-tls"\]$/default = ["native-tls", "bon"]/' sdk/Cargo.toml
rm -f sdk/Cargo.toml.bak

# Enum query params already carry the models:: prefix the template adds.
find sdk/src/apis -name '*.rs' -exec sed -i.bak 's/models::models::/models::/g' {} +
find sdk/src/apis -name '*.bak' -delete
rm -f sdk/.travis.yml
