# AERES Stylus Token

AERES - ERC20 Arbitrum Stylus Token smart contract

## Requirements

Install the Stylus CLI tool with Cargo

```bash
cargo install --force cargo-stylus cargo-stylus-check
```

Add the `wasm32-unknown-unknown` build target to your Rust compiler:

```
rustup target add wasm32-unknown-unknown
```

You should now have it available as a Cargo subcommand:

```bash
cargo stylus --help
```

Also you may need `solc`: 
```bash
sudo add-apt-repository ppa:ethereum/ethereum
sudo apt-get update
sudo apt-get install solc
```


## Build


Parameters like Name and Symbol may be set while building the contract using env variables (NAME, SYMBOL). If not set the defaults are "Test Arbitrum Stylus Token", "SMB" respectively. Decimals parameter is static and set to 18.

Create `res` folder for artifacts:

```bash
mkdir arbitrum-token-sc/res
```


### Make Wasm 
Build wasm file: 
```bash
cargo build --release --target=wasm32-unknown-unknown
```
Optimize it via command 
```bash
cargo build --release --target=wasm32-unknown-unknown
```



### ABI Export

### ABI Export

You can export the Solidity ABI for your program by using the `cargo stylus` tool as follows:

```bash
cargo stylus export-abi
```


Exporting ABIs uses a feature that is enabled by default in your Cargo.toml:

```toml
[features]
export-abi = ["stylus-sdk/export-abi"]
```

### Artifacts:
* `res/aeres-erc20-token.wasm`
* `res/aeres-erc20-token.sol`

### Examples:
* `NAME="AERES" SYMBOL="ARS" cargo make wasm`
