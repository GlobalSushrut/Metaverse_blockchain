use serde::{Serialize, Deserialize};
use blake3;
use crate::math::precision::PreciseFloat;

/// Represents a cryptographic tally over system state
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TallyResult {
    /// The resulting hash of the tally computation
    pub hash: [u8; 32],
    /// Number of operations processed
    pub operation_count: u64,
}

/// Computes cryptographic tallies over quantum state transitions
pub struct TallyComputer {
    /// Current hash state
    current_hash: [u8; 32],
    /// Previous hash state for verification
    previous_hash: [u8; 32],
    /// Number of operations processed
    operation_count: u64,
    /// Precision for floating point operations
    precision: u8,
}

impl TallyComputer {
    /// Create a new TallyComputer instance
    pub fn new(precision: u8) -> Self {
        Self {
            current_hash: [0u8; 32],
            previous_hash: [0u8; 32],
            operation_count: 0,
            precision,
        }
    }

    /// Computes the tally as:
    ///   T(i) = H( S(i) ⊕ O(i) ) ⊗ P(i)
    /// where:
    ///   - ⊕ is implemented as a byte‑wise XOR between state and operation
    ///   - ⊗ is simulated as a byte‑wise XOR between the hash result and the proof
    pub fn compute_tally(&mut self, state: &[u8], operation: &[u8], proof: &[u8]) -> TallyResult {
        if state.is_empty() || operation.is_empty() || proof.is_empty() {
            return TallyResult {
                hash: self.current_hash,
                operation_count: self.operation_count,
            };
        }

        // Save the current hash for verification
        self.previous_hash = self.current_hash;
        
        // First hash the state
        let state_hash = blake3::hash(state);
        let state_hash_bytes = state_hash.as_bytes();
        
        // Combine with previous hash if not first operation
        let mut state_xor = [0u8; 32];
        if self.operation_count == 0 {
            state_xor.copy_from_slice(state_hash_bytes);
        } else {
            for i in 0..32 {
                state_xor[i] = state_hash_bytes[i] ^ self.previous_hash[i];
            }
        }
        
        // Then combine state with operation using XOR
        let mut xor_state = [0u8; 32];
        for i in 0..32 {
            let state_byte = state_xor[i];
            let op_byte = operation[i % operation.len()];
            xor_state[i] = state_byte ^ op_byte;
        }
        
        // Hash the XORed state
        let hash_result = blake3::hash(&xor_state);
        let hash_bytes = hash_result.as_bytes();
        
        // Normalize proof to 32 bytes
        let proof_fixed: [u8; 32] = if proof.len() == 32 {
            proof.try_into().unwrap()
        } else {
            let hash_proof = blake3::hash(proof);
            *hash_proof.as_bytes()
        };
        
        // Combine with proof
        let mut final_hash = [0u8; 32];
        for i in 0..32 {
            final_hash[i] = hash_bytes[i] ^ proof_fixed[i];
        }
        
        self.current_hash = final_hash;
        self.operation_count += 1;
        
        TallyResult {
            hash: final_hash,
            operation_count: self.operation_count,
        }
    }

    pub fn compute_frc_proof(&self, data: &[u8]) -> PreciseFloat {
        let hash = blake3::hash(data);
        let bytes = hash.as_bytes();
        let mut value: i128 = 0;
        for (i, &b) in bytes.iter().enumerate() {
            if i < self.precision as usize {
                value += (b as i128) * 10i128.pow((self.precision - i as u8 - 1) as u32);
            }
        }
        PreciseFloat::new(value, self.precision)
    }

    pub fn compute_physics_state(&self, current_state: &[u8]) -> PreciseFloat {
        let mut state = [0u8; 32];
        for i in 0..32 {
            state[i] = current_state[i % current_state.len()];
        }
        let mut sum: i128 = 0;
        for (i, &b) in state.iter().enumerate() {
            if i < self.precision as usize {
                sum += (b as i128) * 10i128.pow((self.precision - i as u8 - 1) as u32);
            }
        }
        PreciseFloat::new(sum.abs(), self.precision)
    }

    pub fn compute_ai_decision(&self, data: &[u8]) -> PreciseFloat {
        let hash = blake3::hash(data);
        let bytes = hash.as_bytes();
        let mut value: i128 = 0;
        for (i, &b) in bytes.iter().enumerate().take(16) {
            if i < self.precision as usize {
                value += (b as i128) * 10i128.pow((self.precision - i as u8 - 1) as u32);
            }
        }
        PreciseFloat::new(value.abs(), self.precision)
    }

    /// Verify that an expected tally matches computed one
    pub fn verify_tally(&self, expected: &TallyResult, state: &[u8], operation: &[u8], proof: &[u8]) -> bool {
        // For verification, we need to compute the hash using the same inputs and method
        // First hash the state
        let state_hash = blake3::hash(state);
        let state_hash_bytes = state_hash.as_bytes();
        
        // Combine with previous hash if not first operation
        let mut state_xor = [0u8; 32];
        if expected.operation_count == 1 {
            state_xor.copy_from_slice(state_hash_bytes);
        } else {
            for i in 0..32 {
                state_xor[i] = state_hash_bytes[i] ^ self.previous_hash[i];
            }
        }
        
        // Then combine state with operation using XOR
        let mut xor_state = [0u8; 32];
        for i in 0..32 {
            let state_byte = state_xor[i];
            let op_byte = operation[i % operation.len()];
            xor_state[i] = state_byte ^ op_byte;
        }
        
        // Hash the XOR result
        let hash_xor = blake3::hash(&xor_state);
        let hash_xor_bytes = hash_xor.as_bytes();

        // Normalize proof to 32 bytes
        let proof_fixed: [u8; 32] = if proof.len() == 32 {
            proof.try_into().unwrap()
        } else {
            let hash_proof = blake3::hash(proof);
            *hash_proof.as_bytes()
        };

        // Combine hash with proof using XOR
        let mut computed_hash = [0u8; 32];
        for i in 0..32 {
            computed_hash[i] = hash_xor_bytes[i] ^ proof_fixed[i];
        }
        
        if computed_hash != expected.hash {
            println!("Hash mismatch:\nExpected: {:?}\nComputed: {:?}", expected.hash, computed_hash);
            return false;
        }
        
        // The operation count should match exactly what we expect
        if expected.operation_count != self.operation_count {
            println!("Operation count mismatch:\nExpected: {}\nComputed: {}", 
                expected.operation_count, self.operation_count);
            return false;
        }
        
        true
    }

    /// Get the current tally state
    pub fn get_current_state(&self) -> TallyResult {
        TallyResult {
            hash: self.current_hash,
            operation_count: self.operation_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tally_computation() {
        let mut computer = TallyComputer::new(20);
        
        // Test 1: Initial state transition
        let state1 = b"initial_quantum_state";
        let op1 = b"test_operation_1";
        let proof1 = b"test_proof_data_1";
        
        let result1 = computer.compute_tally(state1, op1, proof1);
        assert_eq!(result1.operation_count, 1, "First operation should have count 1");
        assert_ne!(result1.hash, [0u8; 32], "Hash should not be zero");
        assert!(computer.verify_tally(&result1, state1, op1, proof1), 
                "Failed to verify first state transition");
        
        // Test 2: Second state transition
        let state2 = b"quantum_state_2";
        let op2 = b"test_operation_2";
        let proof2 = b"test_proof_data_2";
        
        let result2 = computer.compute_tally(state2, op2, proof2);
        assert_eq!(result2.operation_count, 2, "Second operation should have count 2");
        assert_ne!(result2.hash, [0u8; 32], "Hash should not be zero");
        assert!(computer.verify_tally(&result2, state2, op2, proof2),
                "Failed to verify second state transition");
        
        // Test 3: Empty inputs
        let empty_result = computer.compute_tally(&[], op1, proof1);
        assert_eq!(empty_result.hash, result2.hash, "Empty state should return current hash");
        
        let empty_result = computer.compute_tally(state1, &[], proof1);
        assert_eq!(empty_result.hash, result2.hash, "Empty operation should return current hash");
        
        let empty_result = computer.compute_tally(state1, op1, &[]);
        assert_eq!(empty_result.hash, result2.hash, "Empty proof should return current hash");
        
        // Test 4: Verify hash chain properties
        assert_ne!(result1.hash, result2.hash, "Consecutive states should have different hashes");
        
        // Test 5: Verify deterministic property
        let mut computer2 = TallyComputer::new(20);
        // First apply the same initial state transition
        let repeat1 = computer2.compute_tally(state1, op1, proof1);
        assert_eq!(result1.hash, repeat1.hash, "Same inputs should produce same hash");
        
        // Then verify the second transition produces the same result
        let repeat2 = computer2.compute_tally(state2, op2, proof2);
        assert_eq!(result2.hash, repeat2.hash, "Same sequence should produce same hash");
        
        // Test 6: Physics state computation
        let physics_state = computer.compute_physics_state(state1);
        assert!(physics_state.value > 0, "Physics state should be positive");
        
        // Test 7: FRC proof computation
        let frc_proof = computer.compute_frc_proof(proof1);
        assert!(frc_proof.value > 0, "FRC proof should be positive");
        
        // Test 8: AI decision computation
        let ai_decision = computer.compute_ai_decision(state2);
        assert!(ai_decision.value > 0, "AI decision should be positive");
    }
}
