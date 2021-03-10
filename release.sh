./target/release/mathchain-parachain build-spec --chain mathchain-pc1-genesis --raw --disable-default-bootnode > node/res/mathchain-pc1.json
./target/release/mathchain-parachain export-genesis-wasm --chain mathchain-pc1 > node/res/genesis-wasm
./target/release/mathchain-parachain export-genesis-state --parachain-id 40 --chain mathchain-pc1 > node/res/genesis-state