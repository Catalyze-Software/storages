#!/bin/bash

# run it with bash

wasm=wasm/shard_profile.wasm.gz

dfx canister install index_profile --argument '(vec {principal "syzio-xu6ca-burmx-4afo2-ojpcw-e75j3-m67o5-s5bes-5vvsv-du3t4-wae"})' --wasm wasm/index_profile.wasm.gz
dfx canister call index_profile _dev_upload_wasm --argument-file <(echo "(blob \"$(hexdump -ve '1/1 "%.2x"' "$wasm" | sed 's/../\\&/g')\")")
dfx canister call index_profile _dev_extend_shards '2'
