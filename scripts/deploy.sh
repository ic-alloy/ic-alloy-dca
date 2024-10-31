#!/bin/bash

dfx deploy internet_identity

dfx deploy evm_rpc

dfx deploy xrc --with-cycles 100000000000

dfx deploy backend
candid-extractor catts_engine.wasm >../../../packages/catts_engine/catts_engine.did &&
  ic-wasm catts_engine.wasm -o catts_engine.wasm metadata candid:service -f ../../../packages/catts_engine/catts_engine.did -v public &&
  gzip -c catts_engine.wasm >catts_engine.wasm.gz

dfx deploy frontend
