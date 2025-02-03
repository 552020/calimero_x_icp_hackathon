/*
*   Nodes will mine indefinitely, generating new blocks over time.
*   Each node will continue mining unless explicitly stopped.
*
*   # Start Mining a Block
*   meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> mine_block --args '{"miner_id": "node1", "data": "tx-data"}'
*
*   # Stop All Mining Processes
*   meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> stop_mining
*
*   # Resume Mining
*   meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> start_mining
*
*   # Retrieve All Mined Blocks
*   meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> get_mined_blocks
*/


#![cfg_attr(target_arch = "wasm32", no_std)] // ✅ Ensure compatibility only in WASM

extern crate alloc; // ✅ Needed for `String`, `Vec`, and `format!`
use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;

use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;
use sha2::{Sha256, Digest};
use core::sync::atomic::{AtomicU64, Ordering}; // ✅ Atomic counter for nonce

/// **Global Atomic Nonce Counter (Ensures Unique Nonces Without Randomness)**
static NONCE: AtomicU64 = AtomicU64::new(0);

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningState {
    blocks: UnorderedMap<String, String>, // Store mined blocks
    difficulty: u32,  // Mining difficulty
    is_mining_active: bool,  // ✅ Mining status flag
}

#[app::logic]
impl MiningState {
    #[app::init]
    pub fn init() -> MiningState {
        env::log("Initializing mining state in Calimero context.");
        MiningState {
            blocks: UnorderedMap::new(),
            difficulty: 4, // Adjust difficulty for mining
            is_mining_active: true, // ✅ Start mining by default
        }
    }

    /// **Compute SHA-256 hash of the given input**
    fn compute_sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// **Generate a Nonce Using an Atomic Counter**
    fn generate_nonce() -> u64 {
        NONCE.fetch_add(1, Ordering::Relaxed) // ✅ Ensures thread safety without `getrandom`
    }

    /// **Mining Function with Unique Block Hashing**
    pub fn mine_block(&mut self, miner_id: String, data: String, max_iterations: Option<u64>) -> Result<Option<String>, Error> {
        let max_iters = max_iterations.unwrap_or(1_000_000); // ✅ Default to 1,000,000 if missing
        let mut nonce: u64 = Self::generate_nonce(); // ✅ No randomness, just unique count

        env::log(&format!("Miner {} started mining with max_iterations = {}...", miner_id, max_iters));

        for _ in 0..max_iters {
            if !self.is_mining_active {
                env::log(&format!("Mining stopped for miner {}.", miner_id));
                return Ok(None);
            }

            let input = format!("{}{}{}", miner_id, data, nonce); // ✅ Removed timestamp to avoid conflicts
            let hash = Self::compute_sha256(&input);

            if nonce % 100_000 == 0 {
                env::log(&format!("Miner {} working... Nonce: {}", miner_id, nonce));
            }

            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);

                if self.blocks.get(&block_id)?.is_some() {
                    env::log(&format!("Duplicate block detected by miner {}. Restarting mining...", miner_id));
                    return Ok(None);
                }

                self.blocks.insert(block_id.clone(), data.clone())?;
                env::log(&format!("EVENT: block_mined, miner: {}, block: {}", miner_id, block_id));

                return Ok(Some(block_id));
            }

            nonce += 1;
        }

        env::log(&format!("Miner {} reached max iterations. Resuming next cycle...", miner_id));
        Ok(None)
    }

    /// **Stop all mining processes**
    pub fn stop_mining(&mut self) -> Result<(), Error> {
        env::log("Stopping all mining processes...");
        self.is_mining_active = false;
        Ok(())
    }

    /// **Resume mining processes**
    pub fn start_mining(&mut self) -> Result<(), Error> {
        env::log("Resuming mining processes...");
        self.is_mining_active = true;
        Ok(())
    }

    /// **Retrieve all mined blocks from storage**
    pub fn get_mined_blocks(&self) -> Result<Vec<String>, Error> {
        env::log("Fetching all mined blocks...");

        let mut mined_blocks = Vec::new();
        for (block_id, _) in self.blocks.entries()? {
            mined_blocks.push(block_id);
        }

        Ok(mined_blocks)
    }
}




/*
use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;
use sha2::{Sha256, Digest}; // Import SHA-256 hashing functionality


/// **Simulated Blockchain Miner**
#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MinerState {
    blocks: UnorderedMap<String, String>, // Store mined blocks
    difficulty: u32,  // The mining difficulty level
}

#[app::logic]
impl MinerState {
    /// **Initialize the Miner for All Nodes**
    #[app::init]
    pub fn init() -> MinerState {
        env::log("Initializing miner simulation...");
        MinerState {
            blocks: UnorderedMap::new(),
            difficulty: 4, // Set mining difficulty (higher = slower mining)
        }
    }

    /// **Computes a SHA-256 hash of the given input string**
    pub fn compute_sha256(input: &str) -> String {  // 👈 Add `pub` so it can be called
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result) // Convert bytes to hex string
    }

    /// **Simulate a Mining Process**
    // pub fn mine_block(&mut self, miner_id: String, data: String) -> Result<String, Error> {
    //     env::log(&format!("Miner {} is attempting to mine a block...", miner_id));

    //     let mut nonce = 0;
    //     loop {
    //         let input = format!("{}{}{}", miner_id, data, nonce);
    //         let hash = Self::compute_sha256(&input);

    //         // Check if hash meets difficulty criteria (leading zeros)
    //         if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
    //             let block_id = format!("block-{}", hash);
    //             self.blocks.insert(block_id.clone(), data.clone())?;

    //             // Emit event so other nodes know a block was mined
    //             env::log(&format!("EVENT: block_mined, miner: {}, block: {}", miner_id, block_id));

    //             return Ok(block_id);
    //         }

    //         nonce += 1;
    //     }
    // }   

    pub fn mine_block(&mut self, miner_id: String, data: String) -> Result<String, Error> {
        env::log(&format!("Miner {} is starting mining...", miner_id));
    
        let mut nonce = 0;
        let mut blocks_mined = 0;
    
        loop {
            let input = format!("{}{}{}", miner_id, data, nonce);
            let hash = Self::compute_sha256(&input);
    
            // **Only log every 10,000 iterations to prevent overflow**
            if nonce % 10_000 == 0 {
                env::log(&format!("Miner {} progressing... Nonce: {}", miner_id, nonce));
            }
    
            // **Check if hash meets difficulty criteria**
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.blocks.insert(block_id.clone(), data.clone())?;
                blocks_mined += 1;
    
                // ✅ **Only log when a block is mined**
                env::log(&format!("EVENT: block_mined, miner: {}, block: {}, total blocks: {}", miner_id, block_id, blocks_mined));
    
                if blocks_mined >= 5 {
                    env::log(&format!("Miner {} stopping after mining {} blocks.", miner_id, blocks_mined));
                    break;
                }
            }
    
            nonce += 1;
    
            // ✅ **Stop mining after a max nonce to avoid infinite mining**
            if nonce > 1_000_000 {
                env::log(&format!("Miner {} reached max nonce limit. Stopping mining.", miner_id));
                break;
            }
        }
    
        Ok(format!("Miner {} completed mining {} blocks.", miner_id, blocks_mined))
    }
    

    /// **Retrieve Latest Mined Block**
    pub fn get_latest_block(&self) -> Result<Option<String>, Error> {
        env::log("Fetching latest mined block...");
        let latest_block = self.blocks.entries()?.last();    
        Ok(latest_block.map(|(block_id, _data)| block_id))
    }
    
}*/