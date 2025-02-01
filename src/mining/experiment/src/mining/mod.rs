use log::{error, info};

use sha2::{Digest, Sha256};

pub fn mine_block(difficulty: u32, target: u32) -> u128 {
    let mut nonce = 0u128;
    let target: u128 = target.into();
    let difficulty: u128 = difficulty.into();

    info!(
        "Starting mining with difficulty {} and target {}",
        difficulty, target
    );

    while nonce < u128::MAX {
        // Use a hash function to simulate the mining process
        let hash_result = simple_hash(nonce);

        // Log progress
        if nonce % 100 == 0 {
            info!("Testing nonce: {}", nonce);
        }

        // Compare the hash result (u128) with the target and difficulty
        if hash_result % target == difficulty {
            info!("Found valid nonce: {}", nonce);
            return nonce;
        }
        nonce += 1;
    }

    error!("Mining failed: No valid nonce found before max limit");
    nonce
}

fn simple_hash(nonce: u128) -> u128 {
    let mut hasher = Sha256::new();
    hasher.update(nonce.to_le_bytes());
    let result = hasher.finalize();

    let mut hash_bytes = [0u8; 16]; // 128 bits = 16 bytes
    hash_bytes.copy_from_slice(&result[..16]);

    u128::from_le_bytes(hash_bytes)
}
