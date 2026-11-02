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
# The spec declares security schemes but applies them to no operation, so the
# generator would omit credentials from every request.
yq -i '
  .servers = [{"url": "https://api.rixl.com"}] |
  del(.. | select(tag == "!!map") | .not) |
  .components.schemas["google.protobuf.Value"] |= del(.oneOf) |
  .paths |= with_entries(.key |= sub("\{(\w+)\.(\w+)\}"; "{${1}_${2}}")) |
  (.paths[][].parameters[]? | select(.in == "path") | .name) |= sub("\."; "_") |
  (.paths[][].operationId | select(. != null)) |= sub("^.*\."; "") |
  (.paths[][] | select(tag == "!!map" and has("operationId"))).security = [{"Bearer": []}, {"ApiKeyAuth": []}]
' openapi.yaml

# Preserve existing crate version if sdk/Cargo.toml exists
CURRENT_VERSION=""
if [ -f sdk/Cargo.toml ]; then
  CURRENT_VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' sdk/Cargo.toml | head -n1)
fi

rm -rf sdk
$OPENAPI_GENERATOR generate \
  -i openapi.yaml \
  -g rust \
  -o sdk \
  -c config.yaml \
  -t templates \
  ${CURRENT_VERSION:+--additional-properties "packageVersion=${CURRENT_VERSION}"} \
  --skip-validate-spec

# Configure reqwest with rustls, bon as direct dependency, and metadata
sed -i.bak 's/^reqwest = .*/reqwest = { version = "^0.13", default-features = false, features = ["rustls", "json", "multipart", "stream", "query", "form"] }/' sdk/Cargo.toml
sed -i.bak 's/^bon = .*/bon = "2.3"/' sdk/Cargo.toml
sed -i.bak '/^\[features\]/,$d' sdk/Cargo.toml
perl -pi -e 's/^edition = "2021"$/edition = "2021"\nreadme = "README.md"\nkeywords = ["rixl", "sdk", "api"]\ncategories = ["api-bindings", "web-programming"]/' sdk/Cargo.toml
rm -f sdk/Cargo.toml.bak

# Enum query params already carry the models:: prefix the template adds.
find sdk/src/apis -name '*.rs' -exec sed -i.bak 's/models::models::/models::/g' {} +
find sdk/src/apis -name '*.bak' -delete
rm -f sdk/.travis.yml

# Replace generated sdk/README.md with the root curated README.md
cp README.md sdk/README.md
