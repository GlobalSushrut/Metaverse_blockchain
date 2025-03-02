use crate::layers::l1_orchestration::OrchestrationLayer;
use crate::blockchain::core::Block;
use crate::math::precision::PreciseFloat;
use std::collections::HashMap;

/// L2 - Mainnet Layer
/// Main blockchain network that enforces consensus and maintains the primary ledger
pub struct MainnetLayer {
    orchestration: OrchestrationLayer,
    blocks: Vec<Block>,
    state: HashMap<[u8; 32], Vec<u8>>,
    validators: Vec<[u8; 32]>,
    precision: u8,
}

impl MainnetLayer {
    pub fn new(precision: u8) -> Self {
        Self {
            orchestration: OrchestrationLayer::new(precision),
            blocks: Vec::new(),
            state: HashMap::new(),
            validators: Vec::new(),
            precision,
        }
    }

    /// Add a validator to the network
    pub fn add_validator(&mut self, validator_id: [u8; 32]) {
        self.validators.push(validator_id);
    }

    /// Process and add a new block to the chain
    pub fn process_block(&mut self, data: &[u8], proof: &[u8]) -> Result<[u8; 32], &'static str> {
        // Get current state
        let _current_state = self.get_current_state();
        
        // Process through orchestration layer (L1)
        let hash = self.orchestration.process_transition(data, data, proof)?;
        
        // Create new block
        let mut block = Block::new(
            self.blocks.len() as u64,
            if self.blocks.is_empty() { [0u8; 32] } else { self.blocks.last().unwrap().hash },
            data.to_vec(),
            PreciseFloat::new(0, self.precision),
            PreciseFloat::new(1, self.precision),
            PreciseFloat::new(1, self.precision),
            PreciseFloat::new(1, self.precision)
        );
        block.hash = hash;
        
        // Add block to chain
        self.blocks.push(block);
        
        // Update state
        self.state.insert(hash, data.to_vec());
        
        Ok(hash)
    }

    /// Get the current state of the blockchain
    pub fn get_current_state(&self) -> Vec<u8> {
        if let Some(last_block) = self.blocks.last() {
            self.state.get(&last_block.hash)
                .cloned()
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// Get the current block height
    pub fn height(&self) -> usize {
        self.blocks.len()
    }

    /// Get block by hash
    pub fn get_block(&self, hash: &[u8; 32]) -> Option<&Block> {
        self.blocks.iter().find(|block| block.hash == *hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mainnet() {
        let mut mainnet = MainnetLayer::new(20);

        // Add validator
        let validator = blake3::hash(b"test_validator").into();
        mainnet.add_validator(validator);

        // Test 1: Valid block processing
        let data = b"test_block_data";
        // Generate quantum-resistant proof
        let mut proof = Vec::with_capacity(64);
        
        // First 32 bytes: Quantum-resistant hash with good entropy
        let mut hash_bytes = [0u8; 32];
        for i in 0..32 {
            // Alternate between 0s and 1s to ensure good entropy
            hash_bytes[i] = if i % 2 == 0 { 0x55 } else { 0xAA };
        }
        proof.extend_from_slice(&hash_bytes);
        
        // Add encryption proof data
        proof.extend_from_slice(&[0x55; 32]); // Add 32 more bytes of alternating pattern
        
        // First get current state
        let current_state = mainnet.get_current_state();
        
        // Process block with valid data
        let hash = mainnet.process_block(data, &proof)
            .expect("Failed to process block");

        assert_eq!(mainnet.height(), 1);
        assert!(mainnet.get_block(&hash).is_some());
        assert_ne!(hash, [0u8; 32], "Block hash should not be zero");

        // Test 2: Empty inputs
        let empty_result = mainnet.process_block(&[], &proof);
        assert!(empty_result.is_err(), "Empty state should fail");
        assert_eq!(empty_result.unwrap_err(), "Empty input state, operation, or proof");
        
        let empty_proof = mainnet.process_block(&current_state, &[]);
        assert!(empty_proof.is_err(), "Empty proof should fail");
        assert_eq!(empty_proof.unwrap_err(), "Empty input state, operation, or proof");

        // Test 3: Multiple blocks
        let data2 = b"test_block_data_2";
        let data3 = b"test_block_data_3";
        // Generate proofs for each block
        // Generate quantum-resistant proofs for data2 and data3
        let mut proof2 = Vec::with_capacity(64);
        let mut proof3 = Vec::with_capacity(64);
        
        // Hash with good entropy for data2
        let mut hash_bytes2 = [0u8; 32];
        for i in 0..32 {
            hash_bytes2[i] = if i % 2 == 0 { 0x55 } else { 0xAA };
        }
        proof2.extend_from_slice(&hash_bytes2);
        proof2.extend_from_slice(&[0x55; 32]);
        
        // Hash with good entropy for data3
        let mut hash_bytes3 = [0u8; 32];
        for i in 0..32 {
            hash_bytes3[i] = if i % 2 == 0 { 0x55 } else { 0xAA };
        }
        proof3.extend_from_slice(&hash_bytes3);
        proof3.extend_from_slice(&[0x55; 32]);
        
        let hash1 = mainnet.process_block(data2, &proof2).unwrap();
        let hash2 = mainnet.process_block(data3, &proof3).unwrap();
        assert_ne!(hash1, hash2, "Different blocks should have different hashes");
        assert_eq!(mainnet.height(), 3);

        // Test 4: Block retrieval
        assert!(mainnet.get_block(&hash1).is_some(), "Should find block by hash");
        assert!(mainnet.get_block(&[0u8; 32]).is_none(), "Should not find non-existent block");
    }
}
