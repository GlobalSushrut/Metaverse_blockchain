use crate::layers::l0_tally::TallyLayer;
use crate::security::quantum_resistant::QuantumSecurity;

/// L1 - Orchestration Layer
/// Handles governance rules and physics enforcement
pub struct OrchestrationLayer {
    tally: TallyLayer,
    security: QuantumSecurity,
    physics_rules: Vec<PhysicsRule>,
    governance_rules: Vec<GovernanceRule>,
}

pub struct PhysicsRule {
    id: [u8; 32],
    name: String,
    constraint: Box<dyn Fn(&[u8]) -> bool + Send + Sync>,
}

pub struct GovernanceRule {
    id: [u8; 32],
    name: String,
    validator: Box<dyn Fn(&[u8]) -> bool + Send + Sync>,
}

impl OrchestrationLayer {
    pub fn new(precision: u8) -> Self {
        Self {
            tally: TallyLayer::new(),
            security: QuantumSecurity::new(precision),
            physics_rules: Vec::new(),
            governance_rules: Vec::new(),
        }
    }

    /// Add a physics rule to the system
    pub fn add_physics_rule(&mut self, name: &str, constraint: Box<dyn Fn(&[u8]) -> bool + Send + Sync>) -> [u8; 32] {
        let id = blake3::hash(name.as_bytes()).into();
        self.physics_rules.push(PhysicsRule {
            id,
            name: name.to_string(),
            constraint,
        });
        id
    }

    /// Add a governance rule
    pub fn add_governance_rule(&mut self, name: &str, validator: Box<dyn Fn(&[u8]) -> bool + Send + Sync>) -> [u8; 32] {
        let id = blake3::hash(name.as_bytes()).into();
        self.governance_rules.push(GovernanceRule {
            id,
            name: name.to_string(),
            validator,
        });
        id
    }

    /// Process state transition with physics and governance rules
    pub fn process_transition(&mut self, state: &[u8], operation: &[u8], proof: &[u8]) -> Result<[u8; 32], &'static str> {
        // Validate inputs
        if state.is_empty() || operation.is_empty() || proof.is_empty() {
            return Err("Empty input state, operation, or proof");
        }

        // Enhanced transition processing with quantum state verification
        // Hash both state and operation for unique transitions
        let mut hasher = blake3::Hasher::new();
        hasher.update(state);
        hasher.update(operation);
        let state_id = hasher.finalize().into();
        
        // Verify quantum security first
        if !self.security.verify_proof(proof) {
            return Err("quantum security verification failed");
        }
        
        // Apply physics rules
        for rule in &self.physics_rules {
            if !(rule.constraint)(state) {
                return Err("physics rules validation failed");
            }
        }
        
        // Apply governance rules
        for rule in &self.governance_rules {
            if !(rule.validator)(operation) {
                return Err("governance rules validation failed");
            }
        }
        
        Ok(state_id)



    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestration() {
        let mut orchestration = OrchestrationLayer::new(20);

        // Add physics rule: conservation of energy
        orchestration.add_physics_rule(
            "conservation_of_energy",
            Box::new(|state: &[u8]| {
                // Simple example: ensure state length is even and non-empty
                !state.is_empty() && state.len() % 2 == 0
            })
        );

        // Add governance rule: operation size limit
        orchestration.add_governance_rule(
            "operation_size_limit",
            Box::new(|operation: &[u8]| {
                // Example: limit operation size and ensure non-empty
                !operation.is_empty() && operation.len() < 1024
            })
        );

        // Test 1: Valid transition
        let valid_state = b"valid_quantum_state_xx";  // 20 bytes - even length
        let valid_op = b"valid_operation";
        // Generate quantum-resistant proof
        let mut valid_proof = Vec::with_capacity(64);
        
        // First 32 bytes: Quantum-resistant hash with good entropy
        let mut hash_bytes = [0u8; 32];
        for i in 0..32 {
            // Alternate between 0s and 1s to ensure good entropy
            hash_bytes[i] = if i % 2 == 0 { 0x55 } else { 0xAA };
        }
        valid_proof.extend_from_slice(&hash_bytes);
        
        // Add encryption proof data
        valid_proof.extend_from_slice(&[0x55; 32]); // Add 32 more bytes of alternating pattern

        let result = orchestration.process_transition(valid_state, valid_op, &valid_proof);
        assert!(result.is_ok(), "Valid transition should succeed");
        
        // Test 2: Physics rule violation
        let invalid_state = b"invalid_state_x"; // 15 bytes - odd length
        let result = orchestration.process_transition(invalid_state, valid_op, &valid_proof);
        assert!(result.is_err(), "Physics rule violation should be detected");
        assert_eq!(result.unwrap_err(), "physics rules validation failed");

        // Test 3: Governance rule violation
        let large_op = vec![0u8; 2048]; // Operation too large
        let result = orchestration.process_transition(valid_state, &large_op, &valid_proof);
        assert!(result.is_err(), "Governance rule violation should be detected");
        assert_eq!(result.unwrap_err(), "governance rules validation failed");

        // Test 4: Empty inputs
        let result = orchestration.process_transition(&[], valid_op, &valid_proof);
        assert!(result.is_err(), "Empty state should fail");
        assert_eq!(result.unwrap_err(), "Empty input state, operation, or proof");

        let result = orchestration.process_transition(valid_state, &[], &valid_proof);
        assert!(result.is_err(), "Empty operation should fail");
        assert_eq!(result.unwrap_err(), "Empty input state, operation, or proof");

        let result = orchestration.process_transition(valid_state, valid_op, &[]);
        assert!(result.is_err(), "Empty proof should fail");
        assert_eq!(result.unwrap_err(), "Empty input state, operation, or proof");

        // Test 5: Multiple valid transitions
        // First transition
        let result1 = orchestration.process_transition(valid_state, valid_op, &valid_proof).unwrap();
        
        // Second transition with different operation
        let valid_op2 = b"different_operation";
        let result2 = orchestration.process_transition(valid_state, valid_op2, &valid_proof).unwrap();
        assert_ne!(result1, result2, "Different operations should produce different hashes");
    }
}
