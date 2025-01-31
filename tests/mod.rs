//! Blockchain Mining Test Suite
//! ==========================
//!
//! This test suite is more than just verification of code correctness - it's an integral
//! part of the blockchain's consensus mechanism and security model.
//!
//! Key Testing Areas
//! ----------------
//!
//! 1. Mining Difficulty Calibration
//!    - Tests verify that blocks meet the required proof-of-work difficulty
//!    - Benchmarks help adjust difficulty to maintain target block times
//!    - Critical for network stability and security
//!
//! 2. Network Economics
//!    - Mining rewards must be correctly calculated and distributed
//!    - Transaction fees need to incentivize miners appropriately
//!    - Tests ensure the economic model remains viable
//!
//! 3. Security Verification
//!    - Block validation rules prevent various attack vectors
//!    - Double-spending prevention
//!    - Transaction signature verification
//!    - Proof-of-work verification
//!
//! 4. Performance Benchmarking
//!    - Measures mining performance under different conditions:
//!      * Empty blocks vs. blocks with transactions
//!      * Various transaction loads
//!      * Different hardware configurations
//!    - Results used to:
//!      * Adjust difficulty algorithm
//!      * Optimize mining code
//!      * Set appropriate gas limits
//!
//! 5. Edge Cases
//!    - Network forks and reorganizations
//!    - Invalid transactions handling
//!    - Network congestion scenarios
//!
//! Note: Benchmarks are particularly important for difficulty adjustment
//! and should be run on representative hardware.

mod block_tests;
mod core_tests;
mod edge_case_tests;
mod benchmark_tests;

// Re-export common test utilities if needed
pub(crate) mod common {
    use my_blockchain::{Block, Transaction, Miner};
    
    pub fn create_test_block() -> Block {
        // Helper function to create a test block
        Block::new(vec![])
    }

    pub fn create_test_transaction() -> Transaction {
        // Helper function to create a valid test transaction
        Transaction::new()
    }
}
