## build
```
./build.sh
```
## 部署
```
near deploy --wasmFile target/wasm32-unknown-unknown/release/music_nft.wasm --accountId rhythm4nft.testnet
```
## 合约地址
```
https://explorer.testnet.near.org/transactions/3uVUj3F1MtUU4cFDHRdu1W27ZdC8bHf56GDCThtesAuq
```

## init 
```
near call rhythm4nft.testnet new_default_meta '{"owner_id": "rhythm4nft.testnet"}' --account-id rhythm4nft.testnet
```

## nft_mint
```
near call rhythm4nft.testnet nft_mint '{"token_id": "3", "token_owner_id": "'rhythm4nft.testnet'", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "copies": 1}}' --account-id rhythm4nft.testnet --deposit 10
```
## 转移
```
near call rhythm4nft.testnet nft_transfer '{"token_id": "3", "receiver_id": "abel01-test.testnet", "memo": "transfer ownership"}' --accountId rhythm4nft.testnet --deposit 0.000000000000000000000001
```