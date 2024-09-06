#!/bin/bash

env=$1
# run it with bash
dfx identity use catalyze_$env

subjects=(event group notification profile report)

bash scripts/build.sh 
for sub in ${subjects[@]}; do
    dfx canister install index_$sub --network $env --wasm wasm/index_$sub.wasm.gz --mode upgrade
    dfx canister call index_$sub --network $env _dev_upload_wasm --argument-file <(echo "(blob \"$(hexdump -ve '1/1 "%.2x"' "wasm/shard_$sub.wasm.gz" | sed 's/../\\&/g')\")")
    dfx canister call index_$sub --network $env _dev_upgrade_all_shard
done
