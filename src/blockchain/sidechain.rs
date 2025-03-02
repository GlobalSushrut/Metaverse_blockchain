use crate::math::precision::PreciseFloat;
use crate::security::quantum_resistant::QuantumSecurity;
use crate::network::quantum_network::QuantumNetwork;
use crate::orchestration::tally::compute::TallyComputer;
use crate::blockchain::core::Block;
use blake3;
use std::collections::HashMap;

const PROOF_LENGTH: usize = 11; // Length of truncated proof in bytes

pub struct Sidechain {
    chain_id: [u8; 32],
    precision: u8,
    blocks: Vec<Block>,
    state: HashMap<[u8; 32], Vec<u8>>,
    tally_computer: TallyComputer,
    security: QuantumSecurity,
    network: QuantumNetwork,
}

impl Sidechain {
    pub fn new(precision: u8) -> Self {
        let chain_id = blake3::hash(format!("sidechain:{}", precision).as_bytes()).into();
        Self {
            chain_id,
            precision,
            blocks: Vec::new(),
            state: HashMap::new(),
            tally_computer: TallyComputer::new(18), // Using 18 decimal places for high precision
            security: QuantumSecurity::new(precision),
            network: QuantumNetwork::new(precision),
        }
    }

    pub fn get_chain_id(&self) -> [u8; 32] {
        self.chain_id
    }

    fn generate_proof(&self, data: &[u8]) -> Result<Vec<u8>, &'static str> {
        let proof = blake3::hash(data);
        Ok(proof.as_bytes()[..PROOF_LENGTH].to_vec())
    }

    fn compute_block_hash(&self, state: &[u8], proof: &[u8], data: &[u8]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(state);
        hasher.update(proof);
        hasher.update(data);
        *hasher.finalize().as_bytes()
    }

    fn compute_next_state(&self, current_state: &[u8], proof: &[u8], data: &[u8]) -> Vec<u8> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(current_state);
        hasher.update(proof);
        hasher.update(data);
        hasher.finalize().as_bytes().to_vec()
    }

    pub fn add_block(&mut self, data: &[u8]) -> Result<(), &'static str> {
        let current_state = self.get_current_state();
        let proof = self.generate_proof(data)?;

        let block = Block::new(
            self.blocks.len() as u64,
            if self.blocks.is_empty() { [0u8; 32] } else { self.blocks.last().unwrap().hash },
            [&proof[..], data].concat(),
            self.tally_computer.compute_frc_proof(data),
            self.tally_computer.compute_physics_state(&current_state),
            self.tally_computer.compute_ai_decision(data),
            PreciseFloat::new(100, self.precision)
        );

        self.blocks.push(block.clone());
        let next_state = self.compute_next_state(&current_state, &proof, data);
        self.state.insert(block.hash, next_state);

        self.verify_block(&block)?;
        self.network.broadcast_block(&self.blocks.last().unwrap().to_bytes()).ok();
        Ok(())
    }

    pub fn verify_block(&mut self, block: &Block) -> Result<(), &'static str> {
        let mut current_state = vec![0u8; 32];
        if block.index > 0 {
            if let Some(prev_block) = self.blocks.get(block.index as usize - 1) {
                if let Some(state) = self.state.get(&prev_block.hash) {
                    current_state = state.clone();
                }
            }
        }
        
        let (proof, data) = block.data.split_at(PROOF_LENGTH);
        let next_state = self.compute_next_state(&current_state, proof, data);
        if self.state.get(&block.hash) != Some(&next_state) {
            return Err("State transition mismatch");
        }
        Ok(())
    }

    pub fn get_current_state(&self) -> Vec<u8> {
        if let Some(last_block) = self.blocks.last() {
            self.state.get(&last_block.hash)
                .cloned()
                .unwrap_or_else(|| vec![0u8; 32])
        } else {
            vec![0u8; 32]
        }
    }

    pub fn height(&self) -> usize {
        self.blocks.len()
    }

    pub fn validate_chain(&self) -> Result<(), &'static str> {
        let mut current_state = vec![0u8; 32];

        for block in &self.blocks {
            let (proof, data) = block.data.split_at(PROOF_LENGTH);
            let expected_proof = self.generate_proof(data)?;
            if proof != expected_proof {
                return Err("Invalid proof in chain");
            }

            let next_state = self.compute_next_state(&current_state, proof, data);
            if let Some(stored_state) = self.state.get(&block.hash) {
                if next_state != *stored_state {
                    return Err("State transition mismatch");
                }
            } else {
                return Err("Missing state for block");
            }

            current_state = next_state;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proof_generation() -> Result<(), Box<dyn std::error::Error>> {
        let chain = Sidechain::new(8);
        let data = b"test data";
        let proof = chain.generate_proof(data)?;
        assert_eq!(proof.len(), PROOF_LENGTH);
        let proof2 = chain.generate_proof(data)?;
        assert_eq!(proof, proof2);
        Ok(())
    }

    #[test]
    fn test_block_hash_computation() -> Result<(), Box<dyn std::error::Error>> {
        let chain = Sidechain::new(8);
        let data = b"test data";
        let state = vec![0u8; 32];
        let proof = chain.generate_proof(data)?;
        let hash = chain.compute_block_hash(&state, &proof, data);
        assert_eq!(hash.len(), 32);
        let hash2 = chain.compute_block_hash(&state, &proof, data);
        assert_eq!(hash, hash2);
        let state2 = vec![1u8; 32];
        let hash3 = chain.compute_block_hash(&state2, &proof, data);
        assert_ne!(hash, hash3);
        Ok(())
    }

    #[test]
    fn test_state_transition() -> Result<(), Box<dyn std::error::Error>> {
        let mut chain = Sidechain::new(8);
        let data = b"test data";
        chain.add_block(data)?;
        let state = chain.get_current_state();
        assert_eq!(state.len(), 32);
        Ok(())
    }

    #[test]
    fn test_chain_validation() -> Result<(), Box<dyn std::error::Error>> {
        let mut chain = Sidechain::new(8);
        chain.add_block(b"block1")?;
        chain.add_block(b"block2")?;
        chain.validate_chain()?;
        assert_eq!(chain.height(), 2);
        Ok(())
    }

    #[test]
    fn test_sidechain_operations() -> Result<(), Box<dyn std::error::Error>> {
        let mut sidechain = Sidechain::new(8);
        sidechain.add_block(b"test_block_data")?;
        assert_eq!(sidechain.height(), 1);
        let block = sidechain.blocks[0].clone();
        sidechain.verify_block(&block)?;
        let current_state = sidechain.get_current_state();
        assert_eq!(current_state.len(), 32);
        Ok(())
    }
}
