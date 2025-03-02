use crate::math::precision::PreciseFloat;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub previous_hash: [u8; 32],
    pub data: Vec<u8>,
    pub frc_proof: PreciseFloat,
    pub s_physics: PreciseFloat,
    pub ai_decision: PreciseFloat,
    pub quantum_resistance: PreciseFloat,
    pub hash: [u8; 32],
}

impl Block {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap_or_default()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(bytes)
    }

    pub fn new(
        index: u64,
        previous_hash: [u8; 32],
        data: Vec<u8>,
        frc_proof: PreciseFloat,
        s_physics: PreciseFloat,
        ai_decision: PreciseFloat,
        quantum_resistance: PreciseFloat,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
            
        let mut block = Self {
            index,
            timestamp,
            previous_hash,
            data,
            frc_proof,
            s_physics,
            ai_decision,
            quantum_resistance,
            hash: [0; 32],
        };
        
        block.hash = block.calculate_hash();
        block
    }

    fn calculate_hash(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        
        // Combine all block data for hashing
        hasher.update(&self.index.to_le_bytes());
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.previous_hash);
        hasher.update(&self.data);
        hasher.update(&self.frc_proof.value.to_le_bytes());
        hasher.update(&self.s_physics.value.to_le_bytes());
        hasher.update(&self.ai_decision.value.to_le_bytes());
        hasher.update(&self.quantum_resistance.value.to_le_bytes());
        
        let result = hasher.finalize();
        let mut hash = [0; 32];
        hash.copy_from_slice(&result);
        hash
    }
}

#[allow(dead_code)]
pub struct Blockchain {
    chain: Vec<Block>,
    pending_transactions: Vec<Vec<u8>>,
    frc_engine: FRCEngine,
    precision: u8,
}

impl Blockchain {
    pub fn new(precision: u8) -> Self {
        let frc_engine = FRCEngine::new(precision);
        let mut chain = Self {
            chain: Vec::new(),
            pending_transactions: Vec::new(),
            frc_engine,
            precision,
        };
        
        // Create genesis block
        chain.create_genesis_block();
        chain
    }

    fn create_genesis_block(&mut self) {
        let genesis = Block::new(
            0,
            [0; 32],
            b"Genesis Block".to_vec(),
            PreciseFloat::new(1, self.precision),
            PreciseFloat::new(1, self.precision),
            PreciseFloat::new(1, self.precision),
            PreciseFloat::new(1, self.precision),
        );
        self.chain.push(genesis);
    }

    pub fn add_block(&mut self, data: Vec<u8>) -> Result<(), &'static str> {
        let previous_block = self.chain.last().ok_or("Chain is empty")?;
        
        // Calculate all necessary proofs and values
        let frc_proof = self.frc_engine.calculate_proof(self.chain.len());
        let s_physics = self.calculate_physics();
        let ai_decision = self.calculate_ai_decision();
        let quantum_resistance = self.calculate_quantum_resistance();
        
        let new_block = Block::new(
            self.chain.len() as u64,
            previous_block.hash,
            data,
            frc_proof,
            s_physics,
            ai_decision,
            quantum_resistance,
        );
        
        // Verify block before adding
        if self.verify_block(&new_block) {
            self.chain.push(new_block);
            Ok(())
        } else {
            Err("Block verification failed")
        }
    }

    fn verify_block(&self, block: &Block) -> bool {
        // Verify FRC proof
        if !self.frc_engine.verify_proof(&block.frc_proof) {
            return false;
        }
        
        // Verify quantum resistance
        if block.quantum_resistance.value < PreciseFloat::new(95, 2).value {
            return false;
        }
        
        // Verify hash continuity
        if let Some(previous_block) = self.chain.last() {
            if previous_block.hash != block.previous_hash {
                return false;
            }
        }
        
        // Verify block hash
        block.hash == block.calculate_hash()
    }

    fn calculate_physics(&self) -> PreciseFloat {
        // Implementation from physics.rs
        PreciseFloat::new(1, self.precision) // Placeholder
    }

    fn calculate_ai_decision(&self) -> PreciseFloat {
        // Implementation from ai_decision.rs
        PreciseFloat::new(1, self.precision) // Placeholder
    }

    fn calculate_quantum_resistance(&self) -> PreciseFloat {
        // Implementation from quantum.rs
        PreciseFloat::new(95, 2) // 0.95 base resistance
    }
}

pub struct FRCEngine {
    precision: u8,
    factorials: Vec<PreciseFloat>,
}

impl FRCEngine {
    pub fn new(precision: u8) -> Self {
        Self {
            precision,
            factorials: vec![PreciseFloat::new(1, precision)],
        }
    }

    pub fn calculate_proof(&mut self, n: usize) -> PreciseFloat {
        self.ensure_factorial_capacity(n);
        
        let mut sum = PreciseFloat::new(0, self.precision);
        for i in 0..=n {
            sum = sum.add(&self.factorials[i]);
        }
        sum
    }

    pub fn verify_proof(&self, proof: &PreciseFloat) -> bool {
        // Verify that proof follows FRC properties
        proof.value > 0
    }

    fn ensure_factorial_capacity(&mut self, n: usize) {
        while self.factorials.len() <= n {
            let next_factorial = self.factorials.last().unwrap()
                .mul(&PreciseFloat::new(self.factorials.len() as i128, self.precision));
            self.factorials.push(next_factorial);
        }
    }
}
