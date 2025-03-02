#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::precision::PreciseFloat;

    #[test]
    fn test_tally_proof_generation() {
        let mut tally = TallyState::new();
        
        // Create test operation
        let operation = b"test_operation";
        let quantum_state = &[1u8, 2, 3, 4];
        
        // Generate proof
        let proof = tally.compute_tally(operation, quantum_state);
        
        // Verify proof matches operation
        assert!(tally.verify_proof(&proof, operation));
    }

    #[test]
    fn test_quantum_coherence() {
        let mut tally = TallyState::new();
        
        // Create quantum state with high coherence
        let quantum_state = vec![255u8; 32]; // All bits set
        let operation = b"coherent_state";
        
        let proof = tally.compute_tally(operation, &quantum_state);
        let coherence = tally.calculate_coherence(&proof);
        
        // Should have high coherence (close to 1.0)
        assert!(coherence > PreciseFloat::new(900, 3));
    }

    #[test]
    fn test_proof_accumulation() {
        let mut tally = TallyState::new();
        
        // Generate multiple proofs
        let operations = [b"op1", b"op2", b"op3"];
        let quantum_state = &[1u8, 2, 3, 4];
        
        for op in &operations {
            let proof = tally.compute_tally(op, quantum_state);
            tally.accumulate_proof(proof);
        }
        
        // Verify each operation
        for op in &operations {
            let proof = tally.compute_tally(op, quantum_state);
            assert!(tally.verify_proof(&proof, op));
        }
    }

    #[test]
    fn test_state_transition() {
        let mut tally = TallyState::new();
        let initial_state = tally.state_hash;
        
        // Execute operation
        let op = b"state_change";
        let quantum_state = &[1u8, 2, 3, 4];
        let proof = tally.compute_tally(op, quantum_state);
        
        // State should change
        assert_ne!(initial_state, tally.state_hash);
        
        // But proof should still verify
        assert!(tally.verify_proof(&proof, op));
    }
}
