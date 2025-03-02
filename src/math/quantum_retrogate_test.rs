#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::precision::PreciseFloat;

    #[test]
    fn test_quantum_retrogate_initialization() {
        let retrogate = QuantumRetrogate::new(3); // 8 states
        assert_eq!(retrogate.amplitudes.len(), 8);
        assert_eq!(retrogate.phases.len(), 8);
        
        // Check uniform superposition
        let expected_amp = PreciseFloat::new(1000, 3) / PreciseFloat::new(8, 0);
        for amp in retrogate.amplitudes {
            assert_eq!(amp, expected_amp);
        }
    }

    #[test]
    fn test_factorial_phase() {
        let retrogate = QuantumRetrogate::new(3);
        
        // Test factorial phase for state 0
        let phase_0 = retrogate.factorial_phase(0);
        assert_eq!(phase_0, PreciseFloat::new(1, 8));
        
        // Test factorial phase for state 3
        let phase_3 = retrogate.factorial_phase(3);
        assert!(phase_3 > PreciseFloat::new(0, 8));
    }

    #[test]
    fn test_retrogate_coherence() {
        let mut retrogate = QuantumRetrogate::new(2); // 4 states
        
        // Create test amplitudes and phases
        let amplitudes = vec![
            PreciseFloat::new(500, 3),
            PreciseFloat::new(500, 3),
            PreciseFloat::new(500, 3),
            PreciseFloat::new(500, 3),
        ];
        
        let phases = vec![
            PreciseFloat::new(0, 8),
            PreciseFloat::new(785, 3), // ~π/4
            PreciseFloat::new(1571, 3), // ~π/2
            PreciseFloat::new(3142, 3), // ~π
        ];
        
        retrogate.update_state(amplitudes, phases);
        let coherence = retrogate.calculate_retrogate();
        
        // Coherence should be between 0 and 1
        assert!(coherence >= PreciseFloat::new(0, 8));
        assert!(coherence <= PreciseFloat::new(1000, 3));
    }

    #[test]
    fn test_retro_phase_symmetry() {
        let retrogate = QuantumRetrogate::new(2);
        
        // Test phase difference symmetry
        let phase_01 = retrogate.calculate_retro_phase(0, 1);
        let phase_10 = retrogate.calculate_retro_phase(1, 0);
        
        // Should be symmetric
        assert_eq!(phase_01, phase_10);
    }
}
