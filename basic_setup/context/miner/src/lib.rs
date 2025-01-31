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
    
}
