use my_blockchain::{Block, Miner, Transaction};
use super::common;

#[test]
fn test_miner_initialization() {
    let miner = Miner::new();
    assert!(miner.is_ready());
}

#[test]
fn test_basic_mining() {
    let miner = Miner::new();
    let transactions = vec![common::create_test_transaction()];
    let block = miner.mine_block_with_transactions(transactions);
    
    assert!(block.is_valid());
    assert!(block.verify_transactions());
}

#[test]
fn test_mining_difficulty() {
    let miner = Miner::new();
    let block = miner.mine_block();
    
    // Verify the block meets difficulty requirements
    assert!(block.hash().starts_with("0000"));
    assert!(block.verify_difficulty());
}

#[test]
fn test_mining_reward() {
    let miner = Miner::new();
    let initial_balance = miner.get_balance();
    let block = miner.mine_block();
    
    assert!(miner.get_balance() > initial_balance);
    assert_eq!(miner.get_balance(), initial_balance + block.get_mining_reward());
}
