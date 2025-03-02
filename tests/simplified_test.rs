#[cfg(test)]
mod tests {
    use quantum_metaverse::layers::l0_tally::TallyLayer;

    #[test]
    fn test_tally_layer() {
        let mut tally = TallyLayer::new();
        let state = b"test_state";
        let operation = b"test_operation";
        let proof = b"test_proof";
        
        let result = tally.compute_state_transition(state, operation, proof);
        assert!(result.is_ok(), "Tally computation failed");
    }

    #[test]
    fn test_more_tally_operations() {
        let mut tally = TallyLayer::new();
        let state = b"more_test_state";
        let operation = b"more_test_operation";
        let proof = b"more_test_proof";
        
        let result = tally.compute_state_transition(state, operation, proof);
        assert!(result.is_ok(), "More tally computation failed");
    }

    #[test]
    fn test_tally_recovery() {
        let mut tally = TallyLayer::new();
        let state = b"recovery_test_state";
        let operation = b"recovery_test_operation";
        let proof = b"recovery_test_proof";
        
        let result = tally.compute_state_transition(state, operation, proof);
        assert!(result.is_ok(), "Recovery tally computation failed");
        
        // Simulate crash and recovery
        drop(tally);
        
        // Recreate state
        let mut new_tally = TallyLayer::new();
        let new_result = new_tally.compute_state_transition(state, operation, proof);
        assert!(new_result.is_ok(), "Failed to recover tally state");
    }

    #[test]
    fn test_tally_interaction() {
        let mut tally = TallyLayer::new();
        let state = b"interaction_test_state";
        let operation = b"interaction_test_operation";
        let proof = b"interaction_test_proof";
        
        let result = tally.compute_state_transition(state, operation, proof);
        assert!(result.is_ok(), "Interaction tally computation failed");
        
        // Test multiple operations
        let state2 = b"interaction_test_state_2";
        let operation2 = b"interaction_test_operation_2";
        let proof2 = b"interaction_test_proof_2";
        
        let result2 = tally.compute_state_transition(state2, operation2, proof2);
        assert!(result2.is_ok(), "Second interaction tally computation failed");
    }

    #[test]
    fn test_recovery() {
        let mut tally = TallyLayer::new();
        let state = b"recovery_state";
        let operation = b"recovery_operation";
        let proof = b"recovery_proof";
        
        // Create initial state
        let result1 = tally.compute_state_transition(state, operation, proof)
            .expect("Initial computation failed");
            
        // Simulate crash by dropping the instance
        drop(tally);
        
        // Create new instance and verify state can be recreated
        let mut new_tally = TallyLayer::new();
        let result2 = new_tally.compute_state_transition(state, operation, proof)
            .expect("Recovery computation failed");
            
        assert_eq!(result1, result2, "State recovery failed");
    }
}
