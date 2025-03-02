pub mod blockchain;
pub mod network;
pub mod security;
pub mod orchestration;
pub mod crypto;
pub mod layers;

// Re-export security test functions
pub use security::tests::{run_security_tests, run_stress_test, simulate_quantum_attack, perform_network_security_audit};

// Re-export orchestration
pub use orchestration::{Orchestrator, OrchestrationMetrics, RealityLayer, QuantumTally};
pub mod identity;
pub mod governance;
pub mod economics;
pub mod math;
pub mod hubble;
pub mod storage;
pub mod web2;
pub mod web3;
pub mod vm;
