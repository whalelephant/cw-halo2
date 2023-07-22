
build: 
  @echo "Building Optimised WASM file(s)"
  if [[ $(uname -m) =~ "arm64" ]]; then \
  docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    --platform linux/arm64 \
    cosmwasm/workspace-optimizer-arm64:0.13.0; else \
  docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    --platform linux/amd64 \
    cosmwasm/workspace-optimizer:0.13.0; fi

test contract="":
  cargo test --locked --{{ if contract == "" { "workspace" } else { "package" } }} {{contract}} 

generate-schemas: 
  for dir in $PWD/contracts/*/*/; do \
    cd $dir \
    cargo run --example schema \
    cd - \
  done

run-local:
  ./scripts/rm-local-node.sh
  ./scripts/run-test-node.sh

stop-local:
  ./scripts/rm-local-node.sh

  




