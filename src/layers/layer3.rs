use crate::math::precision::PreciseFloat;
use crate::security::quantum_resistant::QuantumSecurity;
use crate::network::quantum_network::QuantumNetwork;
use crate::orchestration::tally::compute::TallyComputer;
use blake3;
use std::collections::HashMap;

pub struct Layer3 {
    precision: u8,
    state_channels: HashMap<[u8; 32], StateChannel>,
    tally_computer: TallyComputer,
    security: QuantumSecurity,
    network: QuantumNetwork,
}

pub struct StateChannel {
    id: [u8; 32],
    balance: PreciseFloat,
    state: Vec<u8>,
    participants: Vec<[u8; 32]>,
}

impl Layer3 {
    pub fn new(precision: u8) -> Self {
        Self {
            precision,
            state_channels: HashMap::new(),
            tally_computer: TallyComputer::new(18), // Using 18 decimal places for high precision
            security: QuantumSecurity::new(precision),
            network: QuantumNetwork::new(precision),
        }
    }

    pub fn create_channel(&mut self, participants: Vec<[u8; 32]>, initial_balance: PreciseFloat) -> Result<[u8; 32], &'static str> {
        let channel_state = format!("init:{}:{}", initial_balance.value, participants.len());
        let channel_id = blake3::hash(channel_state.as_bytes()).into();
        
        let channel = StateChannel {
            id: channel_id,
            balance: initial_balance,
            state: channel_state.into_bytes(),
            participants,
        };
        
        self.state_channels.insert(channel_id, channel);
        Ok(channel_id)
    }

    pub fn update_channel_state(&mut self, channel_id: [u8; 32], new_state: Vec<u8>, proof: &[u8]) -> Result<(), &'static str> {
        let channel = self.state_channels.get_mut(&channel_id)
            .ok_or("Channel not found")?;
            
        // Verify state transition using tally computer
        let result = self.tally_computer.compute_tally(&channel.state, &new_state, proof);
        
        // Verify quantum resistance
        self.security.verify_quantum_resistance(&result.hash)?;
        
        // Update channel state
        channel.state = new_state;
        
        // Broadcast state update
        let serialized = bincode::serialize(&result)
            .map_err(|e| format!("Failed to serialize result: {:?}", e))
            .map_err(|_| "Serialization error")?;
        self.network.broadcast_state(&serialized)?;
        
        Ok(())
    }

    pub fn close_channel(&mut self, channel_id: [u8; 32], final_state: Vec<u8>, signatures: Vec<[u8; 64]>) -> Result<(), &'static str> {
        let channel = self.state_channels.get(&channel_id)
            .ok_or("Channel not found")?;
            
        // Verify all participants have signed
        if signatures.len() != channel.participants.len() {
            return Err("Missing signatures");
        }
        
        // Verify signatures
        for (sig, participant) in signatures.iter().zip(channel.participants.iter()) {
            self.security.verify_signature(participant, &final_state, sig)?;
        }
        
        // Remove channel
        self.state_channels.remove(&channel_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_layer3_operations() {
        let mut layer3 = Layer3::new(20);
        
        // Create test participants
        let participant1 = blake3::hash(b"participant1").into();
        let participant2 = blake3::hash(b"participant2").into();
        let participants = vec![participant1, participant2];
        
        // Test channel creation
        let initial_balance = PreciseFloat::new(1000, 20);
        let channel_id = layer3.create_channel(participants.clone(), initial_balance)
            .expect("Failed to create channel");
            
        // Test state update
        let new_state = b"updated_state".to_vec();
        let proof = b"state_transition_proof";
        layer3.update_channel_state(channel_id, new_state.clone(), proof)
            .expect("Failed to update channel state");
            
        // Verify channel exists
        assert!(layer3.state_channels.contains_key(&channel_id));
    }
}
