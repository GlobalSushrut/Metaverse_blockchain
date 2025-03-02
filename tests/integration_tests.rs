use quantum_metaverse::layers::l0_tally::TallyLayer;

#[tokio::test]
async fn test_tally_operations() -> Result<(), Box<dyn std::error::Error>> {
    let mut tally = TallyLayer::new();
    
    // Test state transition
    let state = b"test_state";
    let operation = b"test_operation";
    let proof = b"test_proof";
    
    let result = tally.compute_state_transition(state, operation, proof);
    assert!(result.is_ok(), "Tally computation failed");
    
    // Test multiple operations
    let state2 = b"test_state_2";
    let operation2 = b"test_operation_2";
    let proof2 = b"test_proof_2";
    
    let result2 = tally.compute_state_transition(state2, operation2, proof2);
    assert!(result2.is_ok(), "Second tally computation failed");
    
    Ok(())
}

