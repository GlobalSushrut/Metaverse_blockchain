use blake3;
use crate::web2::{Web2Runner, Web2AppConfig, Web2AppResult};

/// L0 - Tally Layer
/// Fundamental computation layer that handles quantum state transitions
pub struct TallyLayer {
    current_hash: [u8; 32],
    previous_hash: [u8; 32],
    operation_count: u64,
    web2_runner: Web2Runner,
}

impl TallyLayer {
    pub fn new() -> Self {
        Self {
            current_hash: [0u8; 32],
            previous_hash: [0u8; 32],
            operation_count: 0,
            web2_runner: Web2Runner::new(),
        }
    }

    /// Computes quantum state transition:
    /// T(i) = H(S(i) ⊕ O(i)) ⊗ P(i)
    pub fn compute_state_transition(&mut self, state: &[u8], operation: &[u8], proof: &[u8]) -> Result<[u8; 32], &'static str> {
        if state.is_empty() || operation.is_empty() || proof.is_empty() {
            return Err("Empty input state, operation, or proof");
        }

        // Save current state for verification
        self.previous_hash = self.current_hash;
        
        // Hash the input state first
        let state_hash = blake3::hash(state);
        let state_hash_bytes = state_hash.as_bytes();
        
        // For the first transition, use hashed state
        // For subsequent transitions, combine with current hash
        let state_xor: Vec<u8> = if self.operation_count == 0 {
            state_hash_bytes.to_vec()
        } else {
            self.previous_hash
                .iter()
                .zip(state_hash_bytes.iter())
                .map(|(&a, &b)| a ^ b)
                .collect()
        };

        // XOR with operation
        let state_op_xor: Vec<u8> = state_xor
            .iter()
            .zip(operation.iter().cycle())
            .map(|(&a, &b)| a ^ b)
            .collect();
        
        // Hash the combined state
        let hash_xor = blake3::hash(&state_op_xor);
        let hash_xor_bytes = hash_xor.as_bytes();

        // Normalize proof
        let proof_hash = blake3::hash(proof);
        let proof_bytes = proof_hash.as_bytes();

        // Final combination with proof
        let mut final_hash = [0u8; 32];
        for i in 0..32 {
            final_hash[i] = hash_xor_bytes[i] ^ proof_bytes[i];
        }

        // Update state
        self.current_hash = final_hash;
        self.operation_count += 1;

        Ok(final_hash)
    }

    /// Verify a state transition
    pub fn verify_transition(&self, state: &[u8], operation: &[u8], proof: &[u8], expected_hash: [u8; 32]) -> bool {
        if state.is_empty() || operation.is_empty() || proof.is_empty() {
            return false;
        }

        // Hash the input state first
        let state_hash = blake3::hash(state);
        let state_hash_bytes = state_hash.as_bytes();
        
        // For first transition, use hashed state
        // For subsequent transitions, combine with previous hash
        let state_xor: Vec<u8> = if self.operation_count == 0 {
            state_hash_bytes.to_vec()
        } else {
            self.previous_hash
                .iter()
                .zip(state_hash_bytes.iter())
                .map(|(&a, &b)| a ^ b)
                .collect()
        };

        let state_op_xor: Vec<u8> = state_xor
            .iter()
            .zip(operation.iter().cycle())
            .map(|(&a, &b)| a ^ b)
            .collect();
        
        let hash_xor = blake3::hash(&state_op_xor);
        let hash_xor_bytes = hash_xor.as_bytes();

        let proof_hash = blake3::hash(proof);
        let proof_bytes = proof_hash.as_bytes();

        let mut computed_hash = [0u8; 32];
        for i in 0..32 {
            computed_hash[i] = hash_xor_bytes[i] ^ proof_bytes[i];
        }

        computed_hash == expected_hash
    }

    pub fn get_operation_count(&self) -> u64 {
        self.operation_count
    }

    /// Run a web2 app and record its proof in the quantum state
    pub fn run_web2_app(&mut self, config: Web2AppConfig) -> Result<Web2AppResult, String> {
        // Run the app and get result
        let result = self.web2_runner.run_app(config)?;
        
        // Record proof in quantum state
        self.record_web2_proof(&result)
            .map_err(|e| e.to_string())?;
            
        Ok(result)
    }
    
    /// Record web2 app proof in quantum state
    fn record_web2_proof(&mut self, result: &Web2AppResult) -> Result<(), &'static str> {
        // Create state data from proof and timestamp
        let mut state_data = Vec::new();
        state_data.extend_from_slice(&result.proof);
        state_data.extend_from_slice(&result.timestamp.to_le_bytes());
        
        // Record in quantum state without creating transaction
        self.compute_state_transition(&state_data, &result.output, &result.proof)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_state_transitions() {
        let mut tally = TallyLayer::new();

        // Test state transition
        let state = b"quantum_state";
        let operation = b"quantum_operation";
        let proof = b"quantum_proof";

        let hash = tally.compute_state_transition(state, operation, proof)
            .expect("Failed to compute state transition");

        assert!(tally.verify_transition(state, operation, proof, hash),
                "Failed to verify state transition");
        
        assert_eq!(tally.get_operation_count(), 1);
    }

    #[test]
    fn test_web2_app_execution() {
        let mut tally = TallyLayer::new();
        
        // Create test Python app config
        let config = Web2AppConfig {
            app_id: "test-python".to_string(),
            docker_image: "python:3.9-slim".to_string(),
            command: vec!["python".to_string(), "-c".to_string(), "print('hello')".to_string()],
            env_vars: HashMap::new(),
        };
        
        // Run app and verify result
        let result = tally.run_web2_app(config).unwrap();
        assert!(!result.proof.iter().all(|&x| x == 0));
        assert!(result.timestamp > 0);
        
        // Verify state was updated
        assert!(tally.get_operation_count() > 0);
    }
}
