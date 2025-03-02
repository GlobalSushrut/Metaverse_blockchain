use crate::math::precision::PreciseFloat;
use std::collections::HashMap;

/// Factorial Retrograde Chain Implementation
pub struct FRCBlock {
    previous_hash: [u8; 32],
    transactions: Vec<Transaction>,
    factorial_proof: PreciseFloat,
    retrograde_hash: [u8; 32],
    timestamp: u64,
    depth: u64,
}

pub struct Transaction {
    sender: [u8; 32],
    receiver: [u8; 32],
    amount: PreciseFloat,
    data: Vec<u8>,
    signature: [u8; 64],
}

pub struct FRCChain {
    precision: u8,
    blocks: Vec<FRCBlock>,
    state: HashMap<[u8; 32], AccountState>,
    validation_threshold: PreciseFloat,
}

struct AccountState {
    balance: PreciseFloat,
    nonce: u64,
    last_transaction: u64,
}

impl FRCChain {
    pub fn new(precision: u8) -> Self {
        Self {
            precision,
            blocks: Vec::new(),
            state: HashMap::new(),
            validation_threshold: PreciseFloat::new(95, 2), // 0.95 threshold
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) -> Result<(), &'static str> {
        // Calculate factorial proof
        let proof = self.calculate_factorial_proof(&transactions);
        
        // Validate proof
        if !self.validate_factorial_proof(&proof) {
            return Err("Invalid factorial proof");
        }

        // Create block
        let block = FRCBlock {
            previous_hash: self.get_last_hash(),
            transactions,
            factorial_proof: proof,
            retrograde_hash: self.calculate_retrograde_hash(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            depth: self.blocks.len() as u64,
        };

        // Validate state transition
        if !self.validate_state_transition(&block) {
            return Err("Invalid state transition");
        }

        // Update state
        self.update_state(&block)?;
        
        // Add block
        self.blocks.push(block);
        Ok(())
    }

    fn calculate_factorial_proof(&self, transactions: &[Transaction]) -> PreciseFloat {
        let mut proof = PreciseFloat::new(1, self.precision);
        
        for (i, tx) in transactions.iter().enumerate() {
            // Calculate factorial component: (i + 1)!
            let mut factorial = PreciseFloat::new(1, self.precision);
            for j in 1..=(i + 1) {
                factorial = factorial.mul(&PreciseFloat::new(j as i128, 0));
            }
            
            // Add transaction amount
            let tx_component = factorial.mul(&tx.amount);
            proof = proof.mul(&tx_component);
        }
        
        proof
    }

    fn validate_factorial_proof(&self, proof: &PreciseFloat) -> bool {
        // Validate using mathematical properties
        let chain_depth = PreciseFloat::new(self.blocks.len() as i128, 0);
        let threshold = self.validation_threshold
            .mul(&chain_depth)
            .add(&PreciseFloat::new(1, self.precision));

        proof.value >= threshold.value
    }

    fn calculate_retrograde_hash(&self) -> [u8; 32] {
        let mut hash = [0u8; 32];
        
        if let Some(last_block) = self.blocks.last() {
            // Calculate retrograde hash using previous block's data
            let retrograde_factor = self.calculate_retrograde_factor(last_block);
            
            // Apply retrograde transformation
            for i in 0..32 {
                hash[i] = (last_block.retrograde_hash[i] as f64 * retrograde_factor) as u8;
            }
        }
        
        hash
    }

    fn calculate_retrograde_factor(&self, block: &FRCBlock) -> f64 {
        // Calculate retrograde factor based on block depth and factorial proof
        let depth_factor = (block.depth as f64).ln();
        let proof_factor = block.factorial_proof.value as f64 / 10f64.powi(self.precision as i32);
        
        depth_factor * proof_factor
    }

    fn validate_state_transition(&self, block: &FRCBlock) -> bool {
        let mut temp_state = self.state.clone();
        
        // Validate each transaction
        for tx in &block.transactions {
            if let Some(sender) = temp_state.get_mut(&tx.sender) {
                if sender.balance.value < tx.amount.value {
                    return false;
                }
                sender.balance = sender.balance.sub(&tx.amount);
            } else {
                return false;
            }

            temp_state.entry(tx.receiver)
                .and_modify(|state| state.balance = state.balance.add(&tx.amount))
                .or_insert(AccountState {
                    balance: tx.amount.clone(),
                    nonce: 0,
                    last_transaction: block.timestamp,
                });
        }
        
        true
    }

    fn update_state(&mut self, block: &FRCBlock) -> Result<(), &'static str> {
        for tx in &block.transactions {
            let sender = self.state.get_mut(&tx.sender)
                .ok_or("Sender account not found")?;
            
            sender.balance = sender.balance.sub(&tx.amount);
            sender.nonce += 1;
            sender.last_transaction = block.timestamp;

            self.state.entry(tx.receiver)
                .and_modify(|state| {
                    state.balance = state.balance.add(&tx.amount);
                    state.last_transaction = block.timestamp;
                })
                .or_insert(AccountState {
                    balance: tx.amount.clone(),
                    nonce: 0,
                    last_transaction: block.timestamp,
                });
        }
        
        Ok(())
    }

    fn get_last_hash(&self) -> [u8; 32] {
        self.blocks.last()
            .map(|block| block.retrograde_hash)
            .unwrap_or([0u8; 32])
    }
}
