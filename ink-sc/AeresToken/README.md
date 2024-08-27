# Backend

## Preparation

For building the smart contract found in this folder you will need to have [`cargo-contract`](https://github.com/paritytech/cargo-contract) installed.

```sh
cargo install cargo-contract --force
```

We use the `--force` to update to the most recent `cargo-contract` version.

## Build example contract and generate the contracts metadata

To build a smart contract and generate the Wasm file, navigate to the root of the smart contract and run the following command:

```sh
cargo contract build
```

You should now have an optimized `<contract-name>.wasm` file, a `metadata.json` file and a `<contract-name>.contract` file in the `target` folder of your contract.
The `.contract` file combines the Wasm and metadata into one file and can be used for instantiation.

## Deploy

There are several ways to deploy the contract according to the [documentation](https://use.ink/getting-started/deploy-your-contract/):
1. Using the [Contracts UI](https://contracts-ui.substrate.io/)
2. Using cargo-contract:
```sh
cargo contract upload --suri //Alice
cargo contract instantiate --suri //Alice --args true
```

# Frontend

## Running front end dApp examples

1. Install [nodejs](https://nodejs.org/en/) and then install [pnpm](https://pnpm.io/) `npm install -g pnpm`
2. Install dependencies `pnpm i`
3. Run each example with `pnpm <contract-example-name>`. e.g. `pnpm flipper`
4. Visit [http://localhost:5173](http://localhost:5173) in your browser.

### Commands

* `pnpm basic-contract-caller`
* `pnpm contract-terminate`
* `pnpm contract-transfer`
* `pnpm erc20`
* `pnpm erc721`
* `pnpm flipper`
* `pnpm incrementer`

All examples are built with [useink](https://use.ink/frontend/overview), a React hooks library built by the ink! team.