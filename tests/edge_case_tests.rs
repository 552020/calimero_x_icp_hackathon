
// Blockchain miners face many edge cases that should be tested thoroughly:

// What happens when mining a block with invalid transactions?
// What if a miner tries to mine a block when the network is congested?
// What if two miners find blocks at the same time (forks)?

fn test_mining_with_invalid_transactions() {
    let miner = Miner::new();
    let invalid_transactions = vec![
        Transaction::new_invalid(), // Simulate an invalid transaction
    ];
    let block = miner.mine_block_with_transactions(invalid_transactions);
    
    assert_eq!(block.is_valid(), false); // Should fail due to invalid transactions
}