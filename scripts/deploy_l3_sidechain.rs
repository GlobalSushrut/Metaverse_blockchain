use quantum_metaverse::{
    blockchain::{
        layer3::Layer3,
        sidechain::Sidechain,
    },
    math::precision::PreciseFloat,
    security::quantum_resistant::QuantumSecurity,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Deploying Layer 3 and Sidechain...");
    
    const PRECISION: u8 = 20;
    
    // Initialize Layer 3
    println!("\nInitializing Layer 3...");
    let mut layer3 = Layer3::new(PRECISION);
    
    // Create initial state channels
    let participant1 = blake3::hash(b"validator1").into();
    let participant2 = blake3::hash(b"validator2").into();
    let participants = vec![participant1, participant2];
    
    let initial_balance = PreciseFloat::new(1000000, PRECISION);
    let channel_id = layer3.create_channel(participants, initial_balance)?;
    println!("✓ Layer 3 initialized");
    println!("  Channel ID: {:?}", channel_id);
    
    // Initialize Sidechain
    println!("\nInitializing Sidechain...");
    let mut sidechain = Sidechain::new(PRECISION);
    let chain_id = sidechain.get_chain_id();
    
    // Add genesis block to sidechain
    let genesis_data = b"quantum_metaverse_sidechain_genesis";
    let genesis_proof = b"genesis_quantum_proof";
    sidechain.add_block(genesis_data, genesis_proof)?;
    
    println!("✓ Sidechain initialized");
    println!("  Chain ID: {:?}", chain_id);
    println!("  Genesis block added");
    
    // Verify quantum resistance
    println!("\nVerifying quantum resistance...");
    let security = QuantumSecurity::new(PRECISION);
    security.verify_quantum_resistance(&chain_id)?;
    println!("✓ Quantum resistance verified");
    
    println!("\nDeployment complete!");
    println!("Layer 3 and Sidechain are now operational");
    println!("Current sidechain height: {}", sidechain.height());
    
    Ok(())
}
