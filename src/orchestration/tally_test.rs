#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::precision::PreciseFloat;
    use std::collections::HashMap;

    fn create_test_state_vector() -> QuantumStateVector {
        let amplitudes = vec![
            PreciseFloat::new(500, 3),
            PreciseFloat::new(500, 3),
            PreciseFloat::new(500, 3),
            PreciseFloat::new(500, 3),
        ];
        
        let phases = vec![
            PreciseFloat::new(0, 8),
            PreciseFloat::new(785, 3),
            PreciseFloat::new(1571, 3),
            PreciseFloat::new(3142, 3),
        ];

        QuantumStateVector::new(amplitudes, phases)
    }

    #[test]
    fn test_reality_layer_creation() {
        let mut tally = TallyMetrics::new();
        let layer_id = 1;
        let state = create_test_state_vector();
        
        let result = tally.record_quantum_state(
            [0u8; 32],
            state.clone(),
            layer_id,
            HashMap::new()
        );
        
        assert!(result.is_ok());
        let overlap = result.unwrap();
        assert!(overlap > PreciseFloat::new(0, 8));
        assert!(overlap <= PreciseFloat::new(1000, 3));
    }

    #[test]
    fn test_multiple_reality_layers() {
        let mut tally = TallyMetrics::new();
        let state1 = create_test_state_vector();
        let state2 = create_test_state_vector();
        
        // Record two different reality layers
        let _ = tally.record_quantum_state([0u8; 32], state1, 1, HashMap::new());
        let _ = tally.record_quantum_state([0u8; 32], state2, 2, HashMap::new());
        
        // Check that both layers exist
        assert_eq!(tally.reality_layers.len(), 2);
        
        // Check entanglement exists between layers
        let layer1 = tally.reality_layers.get(&1).unwrap();
        assert!(layer1.entanglement.contains_key(&2));
        
        let layer2 = tally.reality_layers.get(&2).unwrap();
        assert!(layer2.entanglement.contains_key(&1));
        
        // Verify entanglement symmetry
        let ent1_2 = layer1.entanglement.get(&2).unwrap();
        let ent2_1 = layer2.entanglement.get(&1).unwrap();
        assert_eq!(ent1_2, ent2_1);
    }

    #[test]
    fn test_coherence_bounds() {
        let mut tally = TallyMetrics::new();
        let state = create_test_state_vector();
        
        let _ = tally.record_quantum_state([0u8; 32], state, 1, HashMap::new());
        
        let layer = tally.reality_layers.get(&1).unwrap();
        assert!(layer.coherence >= PreciseFloat::new(0, 8));
        assert!(layer.coherence <= PreciseFloat::new(1000, 3));
    }

    #[test]
    fn test_stability_update() {
        let mut tally = TallyMetrics::new();
        let state = create_test_state_vector();
        
        // Record same state twice
        let _ = tally.record_quantum_state([0u8; 32], state.clone(), 1, HashMap::new());
        let _ = tally.record_quantum_state([1u8; 32], state, 1, HashMap::new());
        
        let layer = tally.reality_layers.get(&1).unwrap();
        assert_eq!(layer.observer_count, 2);
        assert!(layer.stability > PreciseFloat::new(0, 8));
    }
}
