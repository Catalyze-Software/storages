#!/bin/bash

env=$1
# run it with bash
dfx identity use catalyze_$env

subjects=(profile group report event)

principal=$(dfx identity get-principal)
echo principal: $principal

sh scripts/build.sh

for sub in ${subjects[@]}; do
    dfx canister install index_$sub --network $env --mode upgrade
done

