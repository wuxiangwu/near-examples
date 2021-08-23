#### 合约调用

    // 添加存证
    near call abel-test.testnet insert_claim '{"claim": "music-hash01"}' --account-id abel02-test.testnet

    // 查看存证
    near call abel-test.testnet get_claim_owner_id '{"claim": "music-hash01"}' --account-id abel02-test.testnet
    
    // 撤销存证
    near call abel-test.testnet revoke_claim '{"claim": "music-hash01"}' --account-id abel02-test.testnet

    // 转移存证
    near call abel-test.testnet transfer_claim '{"claim": "music-hash01","to":"abel01-test.testnet"}' --account-id abel02-test.testnet