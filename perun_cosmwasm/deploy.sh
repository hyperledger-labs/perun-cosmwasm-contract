#!/bin/bash

set -e

source <(curl -sSL https://raw.githubusercontent.com/CosmWasm/testnets/master/pebblenet-1/defaults.env)

# create acc
wasmd keys add wallet

# fund the account from the testnet faucet
JSON=$(jq -n --arg addr $(wasmd keys show -a wallet) '{"denom":"upebble","address":$addr}') && curl -X POST --header "Content-Type: application/json" --data "$JSON" https://faucet.pebblenet.cosmwasm.com/credit

export NODE="--node $RPC"
export TXFLAG="${NODE} --chain-id ${CHAIN_ID} --gas-prices 0.001upebble --gas auto --gas-adjustment 1.3"

#wasmd query wasm list-code $NODE
RES=$(wasmd tx wasm store artifacts/perun_cosmwasm.wasm --from wallet $TXFLAG -y)
echo "If you saw somthing like: 'gas estimate', then it worked!"
