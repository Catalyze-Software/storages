#!/bin/bash

# run it with bash

wasm=wasm/shard_profile.wasm.gz

dfx start --clean --background
dfx deploy index_profile --network development  --identity catalyze_development --with-cycles 10000000000000 --no-wallet '(vec {principal "syzio-xu6ca-burmx-4afo2-ojpcw-e75j3-m67o5-s5bes-5vvsv-du3t4-wae"})'
dfx canister call index_profile _dev_upload_wasm --argument-file <(echo "(blob \"$(hexdump -ve '1/1 "%.2x"' "$wasm" | sed 's/../\\&/g')\")")
dfx canister call index_profile _dev_extend_shards '2'
