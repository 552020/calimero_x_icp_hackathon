/*
*
*   # Start Mining a Block
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> start_mining --args '{"block_data": "block1"}'
*
*   # Miners Join the Pool
*    meroctl --node-name miner1 call --as <EXECUTOR_ID> <CONTEXT_ID> join_mining --args '{"miner_id": "miner1", "hashrate": 100}'
*    meroctl --node-name miner2 call --as <EXECUTOR_ID> <CONTEXT_ID> join_mining --args '{"miner_id": "miner1", "hashrate": 100}'
*
*   # Miners Start Mining
*    meroctl --node-name miner1 call --as <EXECUTOR_ID> <CONTEXT_ID> execute_mining --args '{"miner_id": "miner1"}'
*    meroctl --node-name miner2 call --as <EXECUTOR_ID> <CONTEXT_ID> execute_mining --args '{"miner_id": "miner2"}'
*
*   # Stop Mining
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> stop_mining
*
*   # Retrieve All Mined Blocks
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> get_all_mined_blocks
*
*   # Retrieve Miner Rewards
*    meroctl --node-name manager call --as <EXECUTOR_ID> <CONTEXT_ID> get_miner_rewards --args '{"miner_id": "miner"}'
*/

use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};
use sha2::{Sha256, Digest};

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningPool {
    current_block: Option<String>,
    mined_blocks: UnorderedMap<String, String>,
    worker_stats: UnorderedMap<String, (u64, u64, u64)>,
    active_workers: UnorderedMap<String, bool>,
    difficulty: u32,
    is_mining_active: bool,
    total_hashrate: u64,
    chat_messages: Vector<ChatMessage>,
    chat_sequence: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
struct ChatMessage {
    sender: String,
    message: String,
    sequence: u64,
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        env::log("Initializing mining pool with chat system");
        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4,
            is_mining_active: true,
            total_hashrate: 0,
            chat_messages: Vector::new(),
            chat_sequence: 0,
        }
    }

    // ================ Mining Functionality ================
    
    fn compute_sha256(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn start_mining(&mut self, block_data: String) -> Result<(), Error> {
        if self.is_mining_active {
            return Err(Error::msg("Mining already in progress"));
        }

        self.current_block = Some(block_data.clone());
        self.is_mining_active = true;
        self.worker_stats.clear()?;
        self.active_workers.clear()?;
        self.total_hashrate = 0;

        env::log(&format!("Starting mining for block: {}", block_data));
        Ok(())
    }

    pub fn join_mining(&mut self, miner_id: String, hashrate: u64) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("No active mining session"));
        }

        self.worker_stats.insert(miner_id.clone(), (0, 0, hashrate))?;
        self.active_workers.insert(miner_id.clone(), true)?;
        self.total_hashrate += hashrate;

        env::log(&format!("Miner {} joined with {} H/s", miner_id, hashrate));
        Ok(())
    }

    pub fn execute_mining(&mut self, miner_id: String) -> Result<Option<String>, Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining not active"));
        }

        if !self.active_workers.get(&miner_id)?.unwrap_or(false) {
            return Err(Error::msg("Miner not active"));
        }

        let block_data = match &self.current_block {
            Some(data) => data,
            None => return Err(Error::msg("No active block")),
        };

        if self.mined_blocks.contains(&self.compute_sha256(block_data))? {
            env::log("Block already mined");
            return Ok(None);
        }

        let (mut hash_count, reward, hashrate) = self.worker_stats.get(&miner_id)?.unwrap_or((0, 0, 0));
        let nonce_range = 1_000_000 * hashrate / self.total_hashrate.max(1);
        let mut nonce = 0;

        env::log(&format!("Miner {} starting work on {} hashes", miner_id, nonce_range));

        while nonce < nonce_range {
            let input = format!("{}{}", block_data, nonce);
            let hash = self.compute_sha256(&input);
            hash_count += 1;

            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.mined_blocks.insert(block_id.clone(), block_data.clone())?;
                self.is_mining_active = false;

                let miner_reward = (hash_count * hashrate) / self.total_hashrate.max(1);
                self.worker_stats.insert(miner_id.clone(), (hash_count, miner_reward, hashrate))?;

                env::log(&format!(
                    "EVENT: block_mined,miner:{},block:{},reward:{}",
                    miner_id, block_id, miner_reward
                ));

                return Ok(Some(block_id));
            }

            nonce += 1;
        }

        self.worker_stats.insert(miner_id, (hash_count, reward, hashrate))?;
        Ok(None)
    }

    pub fn stop_mining(&mut self) -> Result<(), Error> {
        self.is_mining_active = false;
        env::log("Mining stopped");
        Ok(())
    }

    // ================ Utility Methods ================

    pub fn get_all_mined_blocks(&self) -> Result<Vec<String>, Error> {
        let mut blocks = Vec::new();
        for (block_id, _) in self.mined_blocks.entries()? { // Using entries instead of keys
            blocks.push(block_id);
        }
        Ok(blocks)
    }

    pub fn get_miner_reward(&self, miner_id: String) -> Result<u64, Error> {
        match self.worker_stats.get(&miner_id)? {
            Some((_, reward, _)) => Ok(reward),
            None => Err(Error::msg("Miner not found")),
        }
    }
    
    // ================ Chat Functionality ================
/* 
    pub fn send_message(&mut self, sender: String, message: String) -> Result<(), Error> {
        let sequence = self.chat_sequence;

        self.chat_messages.push(ChatMessage {
            sender,
            message: message.clone(),
            sequence,
        })?;

        self.chat_sequence += 1;

        env::log(&format!(
            "EVENT: chat_message,sender:{},sequence:{},message:{}",
            sender, sequence, message
        ));

        Ok(())
    }

    pub fn get_messages(&self, from_sequence: u64) -> Result<Vec<ChatMessage>, Error> {
        let mut messages = Vec::new();
        for i in from_sequence..self.chat_sequence {
            if let Ok(Some(msg)) = self.chat_messages.get(i as usize) { // Added Ok wrapper
                messages.push(msg);
            }
        }
        Ok(messages)
    } */

}


/* 
use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::{UnorderedMap, Vector};
use sha2::{Sha256, Digest};
use std::convert::TryInto;
use calimero_sdk::serde::{Serialize, Deserialize};

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningPool {
    current_block: Option<String>, 
    mined_blocks: UnorderedMap<String, String>,
    worker_stats: UnorderedMap<String, (u64, u64, u64)>,
    active_workers: UnorderedMap<String, bool>,
    difficulty: u32, 
    is_mining_active: bool, 
    total_hashrate: u64, 
    chat_messages: Vector<ChatMessage>, 
    chat_sequence: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)] // ✅ Correct placement
#[borsh(crate = "calimero_sdk::borsh")] 
struct ChatMessage {
    sender: String,
    message: String,
    sequence: u64,
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        env::log("Initializing mining pool with chat system.");
        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4, 
            is_mining_active: true, 
            total_hashrate: 0,
            chat_messages: Vector::new(),
            chat_sequence: 0,
        }
    }

    fn compute_sha256(&self, input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn join_mining(&mut self, miner_id: String, hashrate: u64) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining is not active."));
        }
        self.worker_stats.insert(miner_id.clone(), (0, 0, hashrate))?;
        self.active_workers.insert(miner_id.clone(), true)?;
        self.total_hashrate += hashrate;
        env::log(&format!("Miner {} joined the mining pool with hashrate {}.", miner_id, hashrate));
        Ok(())
    }

    pub fn execute_mining(&mut self, miner_id: String) -> Result<Option<String>, Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining is not active."));
        }
        if !self.active_workers.get(&miner_id)?.unwrap_or(false) {
            return Err(Error::msg("Miner is not active."));
        }
        let block_data = self.current_block.clone().ok_or_else(|| Error::msg("No active block."))?;
        let (hash_count, reward, hashrate) = self.worker_stats.get(&miner_id)?.unwrap_or((0, 0, 0));
        let nonce_range = 1_000_000 * hashrate / self.total_hashrate;
        let start_nonce = 0;
        let end_nonce = start_nonce + nonce_range;
        env::log(&format!("Miner {} mining from {} to {}", miner_id, start_nonce, end_nonce));

        let mut nonce = start_nonce;
        loop {
            if !self.is_mining_active {
                env::log(&format!("Mining stopped, miner {} exiting.", miner_id));
                break;
            }
            let input = format!("{}{}", block_data, nonce);
            let hash = self.compute_sha256(&input);
            if nonce % 100_000 == 0 {
                env::log(&format!("Miner {} is working... Nonce: {}", miner_id, nonce));
            }
            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.mined_blocks.insert(block_id.clone(), block_data.clone())?;
                self.is_mining_active = false;
                let miner_reward = (hash_count * hashrate) / self.total_hashrate;
                self.worker_stats.insert(miner_id.clone(), (hash_count, miner_reward, hashrate))?;
                env::log(&format!(
                    "EVENT: block_mined, miner: {}, block: {}, reward: {}",
                    miner_id, block_id, miner_reward
                ));
                return Ok(Some(block_id));
            }
            nonce += 1;
        }
        self.worker_stats.insert(miner_id.clone(), (hash_count, reward, hashrate))?;
        Ok(None)
    }

    pub fn stop_mining(&mut self) -> Result<(), Error> {
        env::log("Stopping mining process...");
        self.is_mining_active = false;
        Ok(())
    }

    pub fn get_all_mined_blocks(&self) -> Result<Vec<String>, Error> {
        env::log("Fetching all mined blocks...");
        let mut blocks = Vec::new();
        for (block_id, _) in self.mined_blocks.entries()? {
            blocks.push(block_id);
        }
        Ok(blocks)
    }

    /// **💬 Send a chat message**
    pub fn send_message(&mut self, sender: String, message: String) -> Result<(), Error> {
        let sequence = self.chat_sequence;

        let chat_message = ChatMessage {
            sender: sender.clone(),
            message: message.clone(),
            sequence,
        };

        self.chat_messages.push(chat_message.clone())?; // ✅ Properly push message
        
        env::log(&format!(
            "EVENT: chat_message, sender: {}, sequence: {}, message: {}",
            chat_message.sender, chat_message.sequence, chat_message.message
        ));

        self.chat_sequence += 1; // ✅ Increment sequence after storing

        Ok(())
    }

    pub fn get_messages(&self, from_sequence: u64) -> Result<Vec<ChatMessage>, Error> {
        env::log("Fetching chat messages...");
        let mut messages = Vec::new();
        let chat_len = self.chat_messages.len()?; // ✅ Properly unwrap `Result`

        for i in from_sequence..chat_len.try_into().unwrap() { // ✅ Convert `usize`
            if let Ok(Some(msg)) = self.chat_messages.get(i.try_into().unwrap()) { // ✅ Convert `usize`
                messages.push(msg);
            }
        }
        Ok(messages)
    }
}
 */


/* use calimero_sdk::borsh::{BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::UnorderedMap;
use sha2::{Sha256, Digest};

// Custom `now` function
pub fn now() -> f64 {
    // Replace this with a custom implementation if needed
    // For now, we'll return a fixed timestamp (e.g., 0.0)
    0.0
}

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct MiningPool {
    current_block: Option<String>, // The block being mined
    mined_blocks: UnorderedMap<String, String>, // Completed blocks
    worker_stats: UnorderedMap<String, (u64, u64, u64)>, // (Start time, hash count, reward)
    active_workers: UnorderedMap<String, bool>, // Track active miners
    difficulty: u32, // Mining difficulty
    is_mining_active: bool, // Is mining currently active?
}

#[app::logic]
impl MiningPool {
    #[app::init]
    pub fn init() -> MiningPool {
        env::log("Initializing mining pool in Calimero context.");
        MiningPool {
            current_block: None,
            mined_blocks: UnorderedMap::new(),
            worker_stats: UnorderedMap::new(),
            active_workers: UnorderedMap::new(),
            difficulty: 4, // Adjust difficulty here
            is_mining_active: false,
        }
    }

    /// **Generate SHA-256 hash**
    fn compute_sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// **Start mining a new block**
    pub fn start_mining(&mut self, block_data: String) -> Result<(), Error> {
        if self.is_mining_active {
            return Err(Error::msg("Mining already in progress."));
        }

        env::log(&format!("Starting mining for block: {}", block_data));
        self.current_block = Some(block_data.clone());
        self.is_mining_active = true;
        self.worker_stats.clear()?;
        self.active_workers.clear()?;

        Ok(())
    }

    /// **Worker joins mining**
    pub fn join_mining(&mut self, miner_id: String) -> Result<(), Error> {
        if !self.is_mining_active {
            return Err(Error::msg("No active mining session."));
        }

        let start_time = (now() / 1000.0) as u64; // Use custom `now` function
        self.worker_stats.insert(miner_id.clone(), (start_time, 0, 0))?;
        self.active_workers.insert(miner_id.clone(), true)?;

        env::log(&format!("Miner {} joined the mining pool.", miner_id));

        Ok(())
    }

    /// **Actual Mining Execution (Each Worker Mines Separately)**
    pub fn execute_mining(&mut self, miner_id: String) -> Result<Option<String>, Error> {
        if !self.is_mining_active {
            return Err(Error::msg("Mining is not active."));
        }

        if self.active_workers.get(&miner_id)?.unwrap_or(false) == false {
            return Err(Error::msg("Miner is not active."));
        }

        let block_data = self.current_block.clone().ok_or(Error::msg("No active block."))?;
        let (start_time, mut hash_count, reward) = self.worker_stats.get(&miner_id)?.unwrap_or((0, 0, 0));

        let mut nonce = 0;
        let worker_count = self.active_workers.entries()?.count() as u64;
        let nonce_range = 1_000_000 / worker_count; // Dynamic nonce assignment
        let start_nonce = nonce * nonce_range;
        let end_nonce = start_nonce + nonce_range;

        env::log(&format!("Miner {} mining from {} to {}", miner_id, start_nonce, end_nonce));

        loop {
            // Check if another miner has already found a block
            if !self.is_mining_active {
                env::log(&format!("Mining stopped, miner {} exiting.", miner_id));
                break;
            }

            let input = format!("{}{}", block_data, nonce);
            let hash = Self::compute_sha256(&input);
            hash_count += 1;

            if nonce % 100_000 == 0 {
                env::log(&format!("Miner {} is working... Nonce: {}", miner_id, nonce));
            }

            if hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                let block_id = format!("block-{}", hash);
                self.mined_blocks.insert(block_id.clone(), block_data.clone())?;
                self.is_mining_active = false; // ❌ Stop all mining

                // Calculate reward
                let total_time = (now() / 1000.0) as u64 - start_time; // Use instant::now() for timestamps
                let miner_reward = (total_time as f64 * 0.5 + hash_count as f64 * 0.5) as u64;
                self.worker_stats.insert(miner_id.clone(), (start_time, hash_count, miner_reward))?;

                env::log(&format!("EVENT: block_mined, miner: {}, block: {}, reward: {}", miner_id, block_id, miner_reward));
                return Ok(Some(block_id));
            }

            nonce += 1;
        }

        self.worker_stats.insert(miner_id.clone(), (start_time, hash_count, reward))?;
        Ok(None)
    }

    /// **Stop mining**
    pub fn stop_mining(&mut self) -> Result<(), Error> {
        env::log("Stopping mining process...");
        self.is_mining_active = false;
        Ok(())
    }

    /// **Retrieve all mined blocks**
    pub fn get_all_mined_blocks(&self) -> Result<Vec<String>, Error> {
        env::log("Fetching all mined blocks...");
        let mut blocks = Vec::new();
        for (block_id, _) in self.mined_blocks.entries()? {
            blocks.push(block_id);
        }
        Ok(blocks)
    }

    /// **Retrieve miner rewards**
    pub fn get_miner_rewards(&self) -> Result<Vec<(String, u64)>, Error> {
        env::log("Fetching all miner rewards...");
        let mut rewards = Vec::new();
        for (miner_id, (_, _, reward)) in self.worker_stats.entries()? {
            rewards.push((miner_id, reward));
        }
        Ok(rewards)
    }
} */