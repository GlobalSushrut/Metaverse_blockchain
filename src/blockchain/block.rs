use crate::math::precision::PreciseFloat;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub prev_hash: [u8; 32],
    pub data: Vec<u8>,
    pub hash: [u8; 32],
    pub nonce: u64,
    pub frc_proof: PreciseFloat,
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
        prev_hash: [u8; 32],
        data: Vec<u8>,
        precision: u8,
    ) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
            
        let mut block = Self {
            index,
            timestamp,
            prev_hash,
            data,
            hash: [0; 32],
            nonce: 0,
            frc_proof: PreciseFloat::new(1, precision),
        };
        
        block.hash = block.calculate_hash();
        block
    }

    pub fn calculate_hash(&self) -> [u8; 32] {
        let mut input = Vec::new();
        input.extend_from_slice(&self.index.to_le_bytes());
        input.extend_from_slice(&self.timestamp.to_le_bytes());
        input.extend_from_slice(&self.prev_hash);
        input.extend_from_slice(&self.data);
        input.extend_from_slice(&self.nonce.to_le_bytes());
        
        blake3::hash(&input).into()
    }

    pub fn verify(&self) -> bool {
        let calculated_hash = self.calculate_hash();
        calculated_hash == self.hash
    }

    pub fn mine(&mut self, difficulty: u8) {
        while !self.is_mined(difficulty) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
    }

    fn is_mined(&self, difficulty: u8) -> bool {
        let target = vec![0u8; (difficulty / 8) as usize];
        self.hash.starts_with(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            0,
            [0u8; 32],
            b"Genesis Block".to_vec(),
            20,
        );
        assert_eq!(block.index, 0);
        assert!(block.verify());
    }

    #[test]
    fn test_block_mining() {
        let mut block = Block::new(
            1,
            [0u8; 32],
            b"Test Block".to_vec(),
            20,
        );
        block.mine(1); // Mine with difficulty 1
        assert!(block.verify());
        assert_eq!(block.hash[0], 0); // First byte should be 0
    }
}
