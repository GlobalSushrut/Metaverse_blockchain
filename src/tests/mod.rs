#[cfg(test)]
mod tests {
    use std::time::Instant;
    use crate::orchestration::tally::{TallyRecorder, QuantumStateVector};
    use crate::orchestration::tally::compute::TallyComputer;
    use crate::math::precision::PreciseFloat;
    use crate::blockchain::core::{Block, Blockchain};
    use crate::vm::executor::{ContractExecutor, Contract, Language};
    use crate::network::quantum_network::QuantumNetwork;
    use crate::security::quantum_resistant::QuantumSecurity;
    use crate::identity::zk_identity::ZKIdentity;
    use crate::governance::ai_governance::AIGovernance;
    use crate::economics::models::EconomicModel;

    const PRECISION: u8 = 20;

    #[test]
    fn test_mathematical_precision() {
        let pi = PreciseFloat::new(314159265358979323846, PRECISION);
        let e = PreciseFloat::new(271828182845904523536, PRECISION);
        
        // Test basic arithmetic
        let sum = pi.add(&e);
        let product = pi.mul(&e);
        assert!(sum.value > 0);
        assert!(product.value > 0);
        
        // Test trigonometric functions
        let cos_pi = pi.cos();
        let sin_pi = pi.sin();
        assert!(cos_pi.value.abs() < 10_i128.pow(PRECISION as u32));
        assert!(sin_pi.value.abs() < 10_i128.pow(PRECISION as u32));
    }

    #[test]
    fn test_blockchain_core() {
        let mut blockchain = Blockchain::new(PRECISION);
        
        // Test block addition
        let data = b"Test Block".to_vec();
        assert!(blockchain.add_block(data.clone()).is_ok());
        
        // Test block verification
        let last_block = blockchain.chain.last().unwrap();
        assert_eq!(last_block.data, data);
        assert!(last_block.frc_proof.value > 0);
    }

    #[test]
    fn test_smart_contract_execution() {
        let mut executor = ContractExecutor::new(PRECISION);
        
        // Test contract execution
        let contract = Contract {
            code: b"function test() { return 42; }".to_vec(),
            language: Language::JavaScript,
            optimization_level: 2,
        };
        
        let result = executor.execute_contract(contract);
        assert!(result.is_ok());
    }

    #[test]
    fn test_quantum_network() {
        let mut network = QuantumNetwork::new(PRECISION);
        
        // Test node addition and entanglement
        let node_a = [0u8; 32];
        let node_b = [1u8; 32];
        
        network.add_node(node_a, QuantumState {
            superposition: PreciseFloat::new(1, PRECISION),
            coherence: PreciseFloat::new(95, 2),
            entanglement_strength: PreciseFloat::new(1, PRECISION),
        });
        
        network.add_node(node_b, QuantumState {
            superposition: PreciseFloat::new(1, PRECISION),
            coherence: PreciseFloat::new(95, 2),
            entanglement_strength: PreciseFloat::new(1, PRECISION),
        });
        
        assert!(network.create_entanglement(node_a, node_b).is_ok());
    }

    #[test]
    fn test_cross_chain_communication() {
        use crate::web3::orchestration::OrchestrationEngine;
        
        // Test cross-chain message passing
        let orchestrator = OrchestrationEngine::new(
            CrossChainMetrics {
                execution_latency: PreciseFloat::new(1, PRECISION),
                state_transition_complexity: PreciseFloat::new(1, PRECISION),
                interoperability_efficiency: PreciseFloat::new(95, 2),
                precision: PRECISION,
            },
            TransactionValidation {
                zk_time: PreciseFloat::new(1, PRECISION),
                consensus_threshold: PreciseFloat::new(95, 2),
                execution_trust: PreciseFloat::new(1, PRECISION),
                precision: PRECISION,
            },
        );
        
        let (score, valid) = orchestrator.process_cross_chain_transaction();
        assert!(valid);
        assert!(score.value > 0);
    }

    #[test]
    fn test_hubble_protocol() {
        use crate::hubble::verification::{ContentVerification, TrustFactorCalculator};
        
        // Test content verification
        let verifier = ContentVerification::new(
            PreciseFloat::new(1, PRECISION),
            PreciseFloat::new(95, 2),
            PreciseFloat::new(1, PRECISION),
            PRECISION,
        );
        
        let (score, verified) = verifier.verify_content();
        assert!(verified);
        assert!(score.value > 0);
        
        // Test trust factor calculation
        let mut calculator = TrustFactorCalculator {
            verification_count: PreciseFloat::new(10, PRECISION),
            malicious_reports: PreciseFloat::new(0, PRECISION),
            source_score: PreciseFloat::new(95, 2),
            precision: PRECISION,
        };
        
        let trust_factor = calculator.calculate_trust_factor();
        assert!(trust_factor.value > 0);
    }

    #[test]
    fn test_quantum_security() {
        let mut security = QuantumSecurity::new(PRECISION);

        // Test key generation
        let (key_id, key) = security.generate_key_pair().unwrap();
        assert!(key.security_level.value > 0);

        // Test encryption/decryption
        let data = b"Test encryption".to_vec();
        let encrypted = security.encrypt(&data, &key_id).unwrap();
        let decrypted = security.decrypt(&encrypted, &key_id).unwrap();
        assert_eq!(data, decrypted);

        // Test security level verification
        let security_level = security.verify_security_level(&key_id).unwrap();
        assert!(security_level.value >= 90); // 0.90 minimum
    }

    #[test]
    fn test_zk_identity() {
        let mut identity_system = ZKIdentity::new(PRECISION);

        // Test identity creation
        let attributes = vec![];
        let (id, identity) = identity_system.create_identity(attributes).unwrap();
        assert!(identity.proof.proof_data.len() > 0);

        // Test identity verification
        let verified = identity_system.verify_identity(&id, &identity.proof).unwrap();
        assert!(verified);

        // Test trust score
        let trust_score = identity_system.get_trust_score(&id).unwrap();
        assert!(trust_score.value >= 70); // 0.70 minimum initial score
    }

    #[test]
    fn test_ai_governance() {
        let mut governance = AIGovernance::new(PRECISION);

        // Test policy creation
        let rules = vec![];
        let weights = vec![];
        let threshold = PreciseFloat::new(90, 2); // 0.90 threshold
        let policy_id = governance.create_policy(rules, weights, threshold).unwrap();

        // Test policy evaluation
        let mut context = std::collections::HashMap::new();
        context.insert("test_metric".to_string(), PreciseFloat::new(95, 2));
        let actions = governance.evaluate_policy(&policy_id, &context).unwrap();

        // Test decision confidence
        let confidence = governance.get_decision_confidence(&policy_id).unwrap();
        assert!(confidence.value >= 0);
    }

    #[test]
    fn test_tally_quantum_resistance() {
        let mut recorder = TallyRecorder::new(PreciseFloat::new(800, 3)); // 0.8 coherence threshold
        
        // Test 1: Basic state recording
        let amplitudes = vec![PreciseFloat::new(707, 3), PreciseFloat::new(707, 3)]; // ~1/√2 each
        let phases = vec![PreciseFloat::new(0, 3), PreciseFloat::new(1571, 3)]; // 0 and π/2
        let result = recorder.record_observation(1, amplitudes.clone(), phases.clone());
        assert!(result.is_ok());
        
        // Test 2: Verify entanglement detection
        let result = recorder.record_observation(2, amplitudes, phases);
        assert!(result.is_ok());
        
        let metrics = recorder.get_metrics();
        assert_eq!(metrics.active_layers, 2);
        assert!(metrics.mean_coherence >= PreciseFloat::new(800, 3));
    }
    
    #[test]
    fn test_tally_state_transitions() {
        let mut computer = TallyComputer::new();
        
        // Create a sequence of state transitions
        let states = vec![
            b"initial state".to_vec(),
            b"quantum state 1".to_vec(),
            b"quantum state 2".to_vec(),
        ];
        
        let mut results = Vec::new();
        
        // Process state transitions
        for (i, state) in states.iter().enumerate() {
            let result = computer.compute_tally(
                state,
                format!("operation {}", i).as_bytes(),
                b"proof"
            );
            results.push(result);
        }
        
        // Verify state transitions
        for (i, result) in results.iter().enumerate() {
            assert!(computer.verify_tally(
                result,
                &states[i],
                format!("operation {}", i).as_bytes(),
                b"proof"
            ));
        }
    }
    
    #[test]
    fn test_tally_performance() {
        let mut recorder = TallyRecorder::new(PreciseFloat::new(800, 3));
        let start = Instant::now();
        let iterations = 1000;
        
        // Perform rapid state transitions
        for i in 0..iterations {
            let amplitudes = vec![
                PreciseFloat::new(707, 3),
                PreciseFloat::new(707, 3),
            ];
            let phases = vec![
                PreciseFloat::new(i as i128 % 628, 3), // Varying phase 0 to 2π
                PreciseFloat::new((i as i128 * 157) % 628, 3),
            ];
            
            let result = recorder.record_observation(i as u32, amplitudes, phases);
            assert!(result.is_ok());
        }
        
        let duration = start.elapsed();
        println!("Processed {} quantum state transitions in {:?}", iterations, duration);
        println!("Average time per transition: {:?}", duration / iterations as u32);
        
        let metrics = recorder.get_metrics();
        assert_eq!(metrics.total_observations as usize, iterations);
    }
    
    #[test]
    fn test_economic_model() {
        let mut model = EconomicModel::new(PRECISION);

        // Test inflation calculation
        let inflation = model.calculate_inflation();
        assert!(inflation.value > 0);

        // Test transaction fee calculation
        let fee = model.calculate_transaction_fee(1000, PreciseFloat::new(50, 2));
        assert!(fee.value > 0);

        // Test staking
        let validator_id = [0u8; 32];
        let stake_amount = PreciseFloat::new(100000, 2); // 1000.00 tokens
        assert!(model.stake_tokens(validator_id, stake_amount).is_ok());

        // Test validator rewards
        let rewards = model.calculate_validator_rewards(&validator_id).unwrap();
        assert!(rewards.value > 0);
    }
}
