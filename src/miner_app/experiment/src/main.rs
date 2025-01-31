use rand::Rng;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct MiningConfig {
    target: u64,
    max_iterations: u64,
    salt: Vec<u8>,
}

impl MiningConfig {
    pub fn new(difficulty: u32) -> Self {
        // Generate a random salt for added complexity
        let mut rng = rand::thread_rng();
        let salt: Vec<u8> = (0..16).map(|_| rng.gen()).collect();

        // Calculate the target based on difficulty

        // 1. Basic Linear Scaling
        // let target = u64::MAX / (difficulty as u64); // Simple scaling based on difficulty

        // 2. Exponential Squared Scaling
        // let target = u64::MAX / (difficulty as u64 * difficulty as u64); // Exponentially harder

        // 3. Exponential with Power of Difficulty
        // let target = u64::MAX / ((difficulty as u64).pow(2)); // Target scales with the square of difficulty

        // 4. Bit Shifting Scaling
        // let target = u64::MAX >> difficulty; // Bit-shift for fast exponential decrease

        // 5. Cubed Difficulty Scaling
        // let target = u64::MAX / (difficulty as u64 * difficulty as u64 * difficulty as u64); // Cubic scaling

        // 6. Custom Exponential Scaling
        let target = u64::MAX / ((difficulty as u64).pow(8)); // Even more aggressive exponential scaling

        // 7. Randomized Target
        // let random_target_offset: u64 = rng.gen_range(1..1000);
        // let target = (u64::MAX / (difficulty as u64)) + random_target_offset; // Adds randomness to target

        // 8. Dynamic Scaling with Time
        // let timestamp = SystemTime::now()
        //     .duration_since(UNIX_EPOCH)
        //     .unwrap()
        //     .as_secs();
        // let target = u64::MAX / (difficulty as u64 + timestamp % 1000); // Includes time-based adjustment

        // 9. Complex Formula
        // let random_factor: u64 = rng.gen_range(1..10);
        // let target = u64::MAX / ((difficulty as u64) * (timestamp % 100) + random_factor); // Multiple factors involved

        // 10. Dynamic Difficulty Adjustment
        // let last_mining_time: u64 = get_last_mining_time(); // Time of last mining attempt
        // let target = u64::MAX / (difficulty as u64 + last_mining_time / 60); // Adjusts based on previous mining time

        Self {
            target,
            max_iterations: u64::MAX,
            salt,
        }
    }
}

pub fn mine_block(config: &MiningConfig) -> Option<(u64, Vec<u8>)> {
    let mut rng = rand::thread_rng();

    for _ in 0..config.max_iterations {
        // Generate a more complex nonce input
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        // Randomize nonce generation
        let nonce: u64 = rng.gen();

        // Create a more complex hash input
        let mut hash_input = Vec::new();
        hash_input.extend_from_slice(&nonce.to_le_bytes());
        hash_input.extend_from_slice(&timestamp.to_le_bytes());
        hash_input.extend_from_slice(&config.salt);

        let hash_result = complex_hash(&hash_input);

        // Check if the hash is valid (smaller than the target)
        if validate_hash(&hash_result, config.target) {
            return Some((nonce, hash_result));
        }
    }

    None
}

fn complex_hash(input: &[u8]) -> Vec<u8> {
    let mut hasher1 = Sha256::new();
    hasher1.update(input);
    let first_hash = hasher1.finalize();

    let mut hasher2 = Sha256::new();
    hasher2.update(&first_hash);

    hasher2.finalize().to_vec()
}

fn validate_hash(hash: &[u8], target: u64) -> bool {
    // Convert the first 8 bytes of the hash to a u64
    let hash_value = u64::from_le_bytes(hash[..8].try_into().unwrap());

    // Check if the hash value is smaller than the target
    hash_value < target
}

fn main() {
    let difficulty = 6; // Adjust difficulty level
    let config = MiningConfig::new(difficulty);

    match mine_block(&config) {
        Some((nonce, hash)) => {
            println!("Mining successful!");
            println!("Nonce: {}", nonce);
            println!(
                "Hash: {}",
                hash.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<String>()
            );
        }
        None => println!("Mining failed to find a valid block"),
    }
}
