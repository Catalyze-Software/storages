#!/bin/bash

env=$1
# run it with bash
dfx identity use catalyze_$env

subjects=(boosted event friend_request group notification profile report topic)

for sub in ${subjects[@]}; do
    dfx build index_$sub --network $env
    dfx canister install index_$sub --network $env --mode upgrade
done
