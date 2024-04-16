#!/bin/sh

./build.sh

if [ $? -ne 0 ]; then
  echo ">> Error building contract"
  exit 1
fi

echo ">> Deploying contract"

# https://docs.near.org/tools/near-cli#near-dev-deploy
# deploy with sub accout
# near deploy --accountId payroll.youraccount.near --wasmFile payroll.wasm

# near dev-deploy  --wasmFile ./target/wasm32-unknown-unknown/release/konnadex_processor.wasm --initFunction init --initArgs '{"invoice_charge": 1,"invoice_amount_converter":100,"owner":"ancestor.testnet"}'
 near dev-deploy --wasmFile ./target/wasm32-unknown-unknown/release/konnadex_multisender.wasm --initFunction init --initArgs '{"salary_charge": 1,"fee_amount_converter":100,"owner":"ancestor.testnet"}'
# near dev-deploy  --wasmFile ./target/wasm32-unknown-unknown/release/konnadex_invoice.wasm --initFunction init --initArgs '{"invoice_charge": 1,"invoice_amount_converter":100,"owner":"ancestor.testnet"}'
