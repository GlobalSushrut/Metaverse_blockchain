use quantum_metaverse::{
    layers::{
        l2_mainnet::MainnetLayer,
    },
    security::quantum_resistant::QuantumSecurity,
};

fn main() {
    println!("Starting Quantum Metaverse Test...");
    
    // Initialize layers with precision of 20 decimal places
    let mut mainnet = MainnetLayer::new(20);
    let mut security = QuantumSecurity::new(20);
    
    // Create some test data
    let test_data = b"Hello Quantum Metaverse!";
    // Generate a quantum-resistant proof using blake3
    let mut hasher = blake3::Hasher::new();
    hasher.update(test_data);
    let test_proof = hasher.finalize().as_bytes().to_vec();
    
    // Process a block
    match mainnet.process_block(test_data, &test_proof) {
        Ok(hash) => {
            println!("Successfully processed block!");
            println!("Block hash: 0x{}", hex::encode(hash));
            
            // Get current state
            let state = mainnet.get_current_state();
            println!("Current state: {:?}", String::from_utf8_lossy(&state));
            
            // Get block height
            println!("Current block height: {}", mainnet.height());
            
            // Get block by hash
            if let Some(block) = mainnet.get_block(&hash) {
                println!("Block data: {:?}", String::from_utf8_lossy(&block.data));
            }
        },
        Err(e) => println!("Error processing block: {}", e),
    }
}
