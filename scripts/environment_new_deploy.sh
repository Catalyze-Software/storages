#!/bin/bash

env=$1
# run it with bash
dfx identity use catalyze_$env

subjects=(profile group report event notification)

proxy_development=puwkw-6qaaa-aaaap-ahmvq-cai
proxy_staging=unset
proxy_production=unset


sh scripts/build.sh

for sub in ${subjects[@]}; do
    shard_wasm=wasm/shard_$sub.wasm.gz
    # dfx canister create index_$sub --network $env --with-cycles 10000000000000
    if [ $env == "development" ]; then
        dfx canister install index_$sub --network $env --argument "(vec {principal \"$proxy_development\"})"
    fi
    if [ $env == "staging" ]; then
        dfx canister install index_$sub --network $env --argument "(vec {principal \"$proxy_staging\"})"
    fi
    if [ $env == "ic" ]; then
        dfx canister install index_$sub --network $env --argument "(vec {principal \"$proxy_production\"})"
    fi
    dfx canister call --network $env index_$sub _dev_upload_wasm --argument-file <(echo "(blob \"$(hexdump -ve '1/1 "%.2x"' "$shard_wasm" | sed 's/../\\&/g')\")")
    dfx canister call --network $env index_$sub _dev_extend_shards '2'
done

