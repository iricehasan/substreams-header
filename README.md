# substreams-header

To install Substreams CLI from:  (https://substreams.streamingfast.io/getting-started/installing-the-cli)

and get an API Key from: (https://app.streamingfast.io/)

You can then export your key:

```bash
export STREAMINGFAST_KEY=server_123123 # Use your own API key
export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)
```

To run the substreams:

First generate Rust codes from the protobuf definitions:
```bash
substreams protogen ./substreams.yaml --exclude-paths="sf/substreams,google"
```

Then compile the module with:
```bash
cargo build --release --target wasm32-unknown-unknown
```

```bash
substreams run -p -e 127.0.0.1:9030 substreams.yaml map_owner
```