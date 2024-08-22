#!/bin/bash

env=$1
# run it with bash
dfx identity use catalyze_$env

subjects=(profile group report event)

principal=$(dfx identity get-principal)
echo principal: $principal

sh scripts/build.sh

for sub in ${subjects[@]}; do
    shard_wasm=wasm/shard_$sub.wasm.gz
    dfx canister create index_$sub --network $env --with-cycles 10000000000000
    dfx canister install index_$sub --network $env --argument "(vec {principal \"$principal\"})"
    dfx canister call --network $env index_$sub _dev_upload_wasm --argument-file <(echo "(blob \"$(hexdump -ve '1/1 "%.2x"' "$shard_wasm" | sed 's/../\\&/g')\")")
    dfx canister call --network $env index_$sub _dev_extend_shards '2'
done

