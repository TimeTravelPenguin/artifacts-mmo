#!/bin/sh

if [ -d "openapi" ]; then
  echo "Error: openapi directory already exists"
  exit 1
fi

openapi-generator generate -i https://api.artifactsmmo.com/openapi.json \
  -o openapi/ \
  -g rust --additional-properties=library=reqwest-trait,supportMiddleware=true,topLevelApiClient=true ||
  exit

# replace all instances of `models::models` with `models`
find . -type f -name "*.rs" -exec sed -i '' 's/models::models/models/g' {} +

cargo clippy --fix --allow-dirty --allow-staged || exit
cargo fmt --all || exit

cd openapi || exit
cargo clippy --fix --allow-dirty --allow-staged || exit
cargo fmt --all
