# Test Arbitrum Stylus Token

ERC20 Arbitrum Stylus Token smart contract

## Build
Public *make* targets:
* `wasm` - produces contract wasm image as well as Solidity ABI under global *res* directory

Parameters like Name and Symbol may be set while building the contract using env variables (NAME, SYMBOL). If not set the defaults are "Test Arbitrum Stylus Token", "SMB" respectively. Decimals parameter is static and set to 18.

### Artifacts:
* `res/arbitrum-erc20-token.wasm`
* `res/arbitrum-erc20-token.sol`

### Examples:
* `NAME="TOKEN NAME" SYMBOL="SMB" cargo make wasm`
