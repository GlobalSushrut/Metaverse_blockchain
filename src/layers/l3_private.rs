use crate::layers::l1_orchestration::OrchestrationLayer;
use crate::blockchain::core::Block;
use crate::math::precision::PreciseFloat;
use std::collections::HashMap;

/// L3 - Private Chain Layer
/// Allows creation of private blockchains that connect to mainnet while following L1 rules
pub struct PrivateChainLayer {
    chain_id: [u8; 32],
    orchestration: OrchestrationLayer,
    blocks: Vec<Block>,
    state: HashMap<[u8; 32], Vec<u8>>,
    owners: Vec<[u8; 32]>,
    mainnet_anchor_points: Vec<[u8; 32]>,
    precision: u8,
}

pub struct ChainConfig {
    pub name: String,
    pub owners: Vec<[u8; 32]>,
    pub initial_state: Vec<u8>,
}

impl PrivateChainLayer {
    pub fn new(config: ChainConfig, precision: u8) -> Self {
        let chain_id = blake3::hash(config.name.as_bytes()).into();
        
        Self {
            chain_id,
            orchestration: OrchestrationLayer::new(precision),
            blocks: Vec::new(),
            state: HashMap::new(),
            owners: config.owners,
            mainnet_anchor_points: Vec::new(),
            precision,
        }
    }

    /// Get the chain's unique identifier
    pub fn get_chain_id(&self) -> [u8; 32] {
        self.chain_id
    }

    /// Process a new block while following L1 rules
    pub fn process_block(&mut self, data: &[u8], proof: &[u8], owner_sig: &[u8; 64]) -> Result<[u8; 32], &'static str> {
        // Verify block is signed by an owner
        self.verify_owner_signature(data, owner_sig)?;
        
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
        
        // Add block
        self.blocks.push(block);
        
        // Update state
        self.state.insert(hash, data.to_vec());
        
        Ok(hash)
    }

    /// Anchor the current state to mainnet
    pub fn anchor_to_mainnet(&mut self, mainnet_block_hash: [u8; 32]) -> Result<(), &'static str> {
        self.mainnet_anchor_points.push(mainnet_block_hash);
        Ok(())
    }

    /// Verify signature from chain owner
    fn verify_owner_signature(&self, _data: &[u8], _signature: &[u8; 64]) -> Result<(), &'static str> {
        // TODO: Implement actual signature verification
        // For now, just check if we have any owners
        if self.owners.is_empty() {
            return Err("No owners registered");
        }
        Ok(())
    }

    /// Get the current state
    pub fn get_current_state(&self) -> Vec<u8> {
        if let Some(last_block) = self.blocks.last() {
            self.state.get(&last_block.hash)
                .cloned()
                .unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// Get the current height of the chain
    pub fn height(&self) -> usize {
        self.blocks.len()
    }

    /// Get the latest mainnet anchor point
    pub fn get_latest_anchor(&self) -> Option<[u8; 32]> {
        self.mainnet_anchor_points.last().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_chain() {
        // Test 1: Chain Creation
        let owner = blake3::hash(b"chain_owner").into();
        let config = ChainConfig {
            name: "test_private_chain".to_string(),
            owners: vec![owner],
            initial_state: b"initial_state".to_vec(),
        };

        let mut private_chain = PrivateChainLayer::new(config, 20);
        let chain_id = private_chain.get_chain_id();
        assert_ne!(chain_id, [0u8; 32], "Chain ID should not be zero");

        // Test 2: Block Processing
        let data = b"private_block_data";
        // Generate valid proof using blake3
        let mut hasher = blake3::Hasher::new();
        hasher.update(data);
        let hash_output = hasher.finalize();
        let proof = hash_output.as_bytes();
        let owner_sig = [1u8; 64]; // Mock valid signature
        
        let hash = private_chain.process_block(data, proof, &owner_sig)
            .expect("Failed to process block");

        assert_eq!(private_chain.height(), 1);
        assert_ne!(hash, [0u8; 32], "Block hash should not be zero");
        
        // Test 3: Empty Inputs
        assert!(private_chain.process_block(&[], proof, &owner_sig).is_err(), "Empty data should fail");
        assert!(private_chain.process_block(data, &[], &owner_sig).is_err(), "Empty proof should fail");
        
        // Test 4: Multiple Blocks
        let data2 = b"private_block_data_2";
        let mut hasher = blake3::Hasher::new();
        hasher.update(data2);
        let hash_output2 = hasher.finalize();
        let proof2 = hash_output2.as_bytes();
        
        let data3 = b"private_block_data_3";
        let mut hasher = blake3::Hasher::new();
        hasher.update(data3);
        let hash_output3 = hasher.finalize();
        let proof3 = hash_output3.as_bytes();
        
        let hash1 = private_chain.process_block(data2, proof2, &owner_sig).unwrap();
        let hash2 = private_chain.process_block(data3, proof3, &owner_sig).unwrap();
        assert_ne!(hash1, hash2, "Different blocks should have different hashes");
        assert_eq!(private_chain.height(), 3);
        
        // Test 5: Mainnet Anchoring
        let mainnet_hash = blake3::hash(b"mainnet_block").into();
        private_chain.anchor_to_mainnet(mainnet_hash)
            .expect("Failed to anchor to mainnet");
        assert_eq!(private_chain.get_latest_anchor(), Some(mainnet_hash));
        
        // Test another anchor point
        let mainnet_hash2 = blake3::hash(b"mainnet_block2").into();
        private_chain.anchor_to_mainnet(mainnet_hash2)
            .expect("Failed to anchor to mainnet");
        assert_eq!(private_chain.get_latest_anchor(), Some(mainnet_hash2));
        
        // Test 6: Invalid Owner
        let config_no_owner = ChainConfig {
            name: "test_chain_no_owner".to_string(),
            owners: vec![],
            initial_state: b"initial_state".to_vec(),
        };
        let mut chain_no_owner = PrivateChainLayer::new(config_no_owner, 20);
        assert!(chain_no_owner.process_block(data, proof, &owner_sig).is_err(), "Chain with no owners should fail block processing");
    }
}
