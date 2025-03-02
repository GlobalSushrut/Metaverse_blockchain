use crate::math::quantum_state::QuantumState;

pub struct DecoherenceModel {
    gamma: f64,  // Decoherence rate
    time_scale: f64,  // Characteristic time scale
}

impl DecoherenceModel {
    pub fn new(gamma: f64, time_scale: f64) -> Self {
        Self {
            gamma,
            time_scale,
        }
    }

    pub fn calculate_decoherence_factor(&self, time: f64) -> f64 {
        (-self.gamma * time / self.time_scale).exp()
    }

    pub fn apply_decoherence(&self, state: &mut QuantumState, time: f64) {
        // If decoherence exceeds threshold, mark as mixed state
        if self.gamma * time > self.time_scale {
            state.is_mixed = true;
        }
    }
}

pub struct QuantumChannelCapacity {
    max_qubits: usize,
    noise_factor: f64,
}

impl QuantumChannelCapacity {
    pub fn new(max_qubits: usize, noise_factor: f64) -> Self {
        Self {
            max_qubits,
            noise_factor,
        }
    }

    pub fn calculate_capacity(&self, input_state: &QuantumState) -> f64 {
        let input_entropy = input_state.calculate_von_neumann_entropy();
        let noise_entropy = self.noise_factor * input_entropy;
        let effective_info = (1.0 - (input_entropy + noise_entropy)).max(0.0);
        effective_info * self.max_qubits as f64
    }

    pub fn calculate_quantum_load(&self, capacity: f64, traffic: f64) -> f64 {
        1.0 / (1.0 + (-capacity / traffic.max(1e-10)).exp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex64;

    #[test]
    fn test_quantum_entropy() {
        // Test maximally mixed state
        let mixed_state = QuantumState::new_maximally_mixed(2);
        let mixed_entropy = mixed_state.calculate_von_neumann_entropy();
        assert!(mixed_entropy >= 0.99 && mixed_entropy <= 1.01, "Mixed state entropy should be close to 1");

        // Test pure state
        let pure_state = QuantumState::new_pure_state(2, vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]);
        let pure_entropy = pure_state.calculate_von_neumann_entropy();
        assert!(pure_entropy >= -0.01 && pure_entropy <= 0.01, "Pure state entropy should be close to 0");
    }

    #[test]
    fn test_decoherence() {
        let mut state = QuantumState::new_pure_state(
            2,
            vec![Complex64::new(1.0/2.0_f64.sqrt(), 0.0), Complex64::new(1.0/2.0_f64.sqrt(), 0.0)]
        );
        let model = DecoherenceModel::new(0.1, 1.0);
        
        let initial_entropy = state.calculate_von_neumann_entropy();
        model.apply_decoherence(&mut state, 1.0);
        let final_entropy = state.calculate_von_neumann_entropy();
        
        assert!(final_entropy >= initial_entropy, "Decoherence should increase entropy");
        assert!(final_entropy <= 1.0, "Entropy should not exceed maximum value");
    }

    #[test]
    fn test_quantum_channel_capacity() {
        // Test with pure state (should have maximum capacity)
        let pure_state = QuantumState::new_pure_state(2, vec![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)]);
        let channel = QuantumChannelCapacity::new(10, 0.1);
        
        let pure_capacity = channel.calculate_capacity(&pure_state);
        let pure_load = channel.calculate_quantum_load(pure_capacity, 1.0);
        
        assert!(pure_capacity >= 9.0, "Pure state should have near-maximum capacity");
        assert!(pure_load >= 0.0 && pure_load <= 1.0, "Load should be normalized");
        
        // Test with mixed state (should have lower capacity)
        let mixed_state = QuantumState::new_maximally_mixed(2);
        let mixed_capacity = channel.calculate_capacity(&mixed_state);
        
        assert!(mixed_capacity < pure_capacity, "Mixed state should have lower capacity than pure state");
    }
}
