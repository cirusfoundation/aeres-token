# Define the target 'build' which accomplishes all tasks
build: res
	cargo build --release --target=wasm32-unknown-unknown 
	cargo stylus export-abi --output ./res/aeres_sc.abi
	cargo stylus export-abi --output ./res/aeres_sc.sol

	cp ./target/wasm32-unknown-unknown/release/aeres_sc.wasm ./res/aeres_sc.wasm

	# wasm-opt -Oz ./target/wasm32-unknown-unknown/release/aeres_sc.wasm --output ./res/aeres_sc.wasm 

	# solc ./res/ERC20.sol -o ./res/aeres_sc.sol --abi --pretty-json --overwrite

	# solc --hashes ./res/ERC20.sol -o ./res/aeres_sc --overwrite

# Step 1: Create the 'res' directory if it doesn't exist
res:
	@mkdir -p ./res
	@mkdir -p ./deploy

stylus-deploy:
	cargo stylus deploy --private-key-path='./deploy/private_key.txt' --endpoint https://arbitrum-sepolia.blockpi.network/v1/rpc/public 
	
# Add a clean target to remove build artifacts
clean:
	cargo clean
	@rm -rf ./res

# Specify that 'res' and 'clean' are phony targets
.PHONY: build res clean