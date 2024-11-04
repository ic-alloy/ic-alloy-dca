#!/bin/bash

.SILENT:

# dfx deploy internet_identity
#
# dfx deploy evm_rpc
#
# dfx deploy xrc --with-cycles 100000000000
#
# dfx deploy backend
#
# dfx deploy frontend

build-backend:
	cd ./target/wasm32-unknown-unknown/release && \
	candid-extractor backend.wasm > ../../../src/backend/backend.did && \
	ic-wasm backend.wasm -o backend.wasm metadata candid:service -f ../../../src/backend/backend.did -v public && \
	gzip -c backend.wasm > backend.wasm.gz

deploy-backend: build-backend
	dfx deploy backend --argument "( \
	  	record { \
					owner = \"$$(dfx identity get-principal)\"; \
					base_token_address = \"0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238\"; \
					base_token_name = \"USDC\"; \
					swap_token_address = \"0xfff9976782d46cc05630d1f6ebab18b2324d6b14\"; \
					swap_token_name = \"WETH\"; \
					fee = 3000; \
					amount_in = 100000; \
					slippage = 5; \
					interval = 3600; \
		} \
	)"


build-frontend:
	dfx generate backend
	cd src/backend && \
	pnpm install && \
	pnpm run build

clean:
	rm -rf .dfx
	rm -rf node_modules
	rm -rf packages/catts_engine/declarations
	rm -rf packages/catts_engine/node_modules
	rm -rf packages/catts_frontend/declarations
	rm -rf packages/catts_frontend/node_modules
	rm -rf packages/catts_frontend/dist
	rm -rf packages/catts_payments/artifacts
	rm -rf packages/catts_payments/cache
	rm -rf packages/catts_payments/coverage
	rm -rf packages/catts_payments/typechain-types
	rm -rf packages/catts_payments/coverage.json
	rm -rf packages/evm_rpc/node_modules
	rm -rf packages/ic_siwe_provider/node_modules
	rm -rf target
	rm -f .env
	cargo clean

