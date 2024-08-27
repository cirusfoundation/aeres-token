# AERES Stylus Token

ERC20 Arbitrum Stylus Token smart contract

## Build


Parameters like Name and Symbol may be set while building the contract using env variables (NAME, SYMBOL). If not set the defaults are "Test Arbitrum Stylus Token", "SMB" respectively. Decimals parameter is static and set to 18.

### Artifacts:
* `res/arbitrum-erc20-token.wasm`
* `res/arbitrum-erc20-token.sol`

### Examples:
* `NAME="AERES" SYMBOL="ARS" cargo make wasm`
