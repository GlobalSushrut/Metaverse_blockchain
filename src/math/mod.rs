pub mod quantum_state;

pub mod precision;
pub mod quantum_entropy;
pub mod entropy;
pub mod physics;
pub mod ai_decision;
pub mod flux;
pub mod quantum_retrogate;

#[cfg(test)]
mod tests {
    use super::*;
    use precision::PreciseFloat;

    #[test]
    fn test_entropy_calculation() {
        let entropy_calc = entropy::EntropyCalculator::new(3);
        let t = PreciseFloat::new(0, 3); // t = 0
        let result = entropy_calc.calculate(t);
        // At t = 0, cos(t) = 1, so S_Entropy = 1 + 0.02 = 1.02
        // With fixed precision (3), expect result in [950, 1050]
        assert!(result.value >= 950 && result.value <= 1050);
    }

    #[test]
    fn test_physics_engine() {
        let engine = physics::PhysicsEngine::new(3);
        let t = PreciseFloat::new(100, 3); // t = 0.1
        let result = engine.s_physics(t);
        // With fixed precision (3), expect result in [950, 1050]
        assert!(result.value >= 950 && result.value <= 1050);
    }

    #[test]
    fn test_ai_decision() {
        let ai_engine = ai_decision::AIDecisionEngine::new(3);
        let t = PreciseFloat::new(100, 3); // t = 0.1
        let result = ai_engine.calculate(t);
        // With fixed precision (3), expect result in [950, 1050]
        assert!(result.value >= 950 && result.value <= 1050);
    }

    #[test]
    fn test_flux_network() {
        let mut network = flux::FluxNetwork::new(5);
        let node = flux::ChaosNode::new(
            PreciseFloat::new(1000, 3), // 1.0 with reduced precision
            PreciseFloat::new(1000, 3)  // 1.0 with reduced precision
        );
        network.add_node(node);
        let flux = network.calculate_flux();
        // With reduced precision, expect ~1.0
        assert!(flux.value >= 950 && flux.value <= 1050);
    }
}
