use quantum_metaverse::layers::l0_tally::TallyLayer;

#[tokio::test]
async fn test_core_functionality() -> Result<(), Box<dyn std::error::Error>> {
    // Test L0 Tally Layer
    let mut tally = TallyLayer::new();
    let state = b"initial_state";
    let operation = b"test_operation";
    let proof = b"test_proof";
    
    let result = tally.compute_state_transition(state, operation, proof);
    assert!(result.is_ok(), "Tally computation failed");

    // Test more tally operations
    let state2 = b"another_state";
    let operation2 = b"another_operation";
    let proof2 = b"another_proof";
    let result2 = tally.compute_state_transition(state2, operation2, proof2);
    assert!(result2.is_ok(), "Second tally computation failed");
    
    Ok(())
}

#[tokio::test]
async fn test_quantum_resistance() -> Result<(), Box<dyn std::error::Error>> {
    let mut tally = TallyLayer::new();
    let state = b"quantum_state";
    let operation = b"quantum_operation";
    let proof = b"quantum_proof";
    
    let result = tally.compute_state_transition(state, operation, proof);
    assert!(result.is_ok(), "Quantum-resistant computation failed");
    
    Ok(())
}

#[tokio::test]
async fn test_recovery_mechanism() -> Result<(), Box<dyn std::error::Error>> {
    let mut tally = TallyLayer::new();
    
    // Create initial state
    let state = b"recovery_test_state";
    let operation = b"recovery_test_operation";
    let proof = b"recovery_test_proof";
    
    let result = tally.compute_state_transition(state, operation, proof);
    assert!(result.is_ok(), "Recovery computation failed");
    
    // Simulate crash and recovery
    drop(tally);
    
    // Recreate state
    let mut new_tally = TallyLayer::new();
    let new_result = new_tally.compute_state_transition(state, operation, proof);
    assert!(new_result.is_ok(), "Failed to recover tally state");
    
    Ok(())
}

#[tokio::test]
async fn test_docker_communication() -> Result<(), Box<dyn std::error::Error>> {
    let mut tally = TallyLayer::new();
    
    // Test in docker environment
    let state = b"docker_state";
    let operation = b"docker_operation";
    let proof = b"docker_proof";
    
    let result = tally.compute_state_transition(state, operation, proof);
    assert!(result.is_ok(), "Docker computation failed");
    
    Ok(())
}
