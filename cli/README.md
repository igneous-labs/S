# cli

CLI binaries and their test utils.

## Notes

### clap v3 vs v4

We're using clap v4 for the workspace, and `sanctum-solana-cli-utils`'s clap feature has version requirement set to `>=3`. For some reason this defaults to v3 despite having v4 in the workspace, so you wil need to edit the `sanctum-solana-cli-utils` entry in `Cargo.lock` to use whatever v4 clap version the workspace has. Hacky, I know.

```toml
[[package]]
name = "sanctum-solana-cli-utils"
version = "0.2.0"
source = "git+https://github.com/igneous-labs/sanctum-solana-utils.git?rev=0147dab#0147dab39083430a440bf83bbdb6e34153c932a8"
dependencies = [
 "async-trait",
 "bincode",
 "clap 2.34.0",
 "clap 3.2.23", # change this to "clap 4.4.18"
 "data-encoding",
 "solana-clap-utils",
 "solana-cli-config",
 "solana-client",
 "solana-sdk",
]
```

### async CLI tests

All async CLI tests need to be annotated with

```rust
#[tokio::test(flavor = "multi_thread")]
```

otherwise timeout will occur when making a RPC request to the test `BanksRpcServer`. Probably some deadlock going on with cloned `BanksClient`. Not investigating further for now.
