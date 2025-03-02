use crate::math::precision::PreciseFloat;

pub struct CrossChainMetrics {
    execution_latency: PreciseFloat,
    state_transition_complexity: PreciseFloat,
    interoperability_efficiency: PreciseFloat,
    precision: u8,
}

impl CrossChainMetrics {
    /// Implements C_Routing = 1/L_Exec × S_Trans × I_Interop
    pub fn calculate_routing_efficiency(&self) -> PreciseFloat {
        let one = PreciseFloat::new(10_i128.pow(self.precision as u32), self.precision);
        
        one.div(&self.execution_latency)
            .mul(&self.state_transition_complexity)
            .mul(&self.interoperability_efficiency)
    }
}

pub struct TransactionValidation {
    zk_time: PreciseFloat,
    consensus_threshold: PreciseFloat,
    execution_trust: PreciseFloat,
    precision: u8,
}

impl TransactionValidation {
    /// Implements V_Transaction = 1/ZK_Time × C_Thresh × E_Trust
    pub fn validate_transaction(&self) -> (PreciseFloat, bool) {
        let one = PreciseFloat::new(10_i128.pow(self.precision as u32), self.precision);
        
        let validation_score = one.div(&self.zk_time)
            .mul(&self.consensus_threshold)
            .mul(&self.execution_trust);
            
        let is_valid = validation_score.value > one.value;
        
        (validation_score, is_valid)
    }
}

pub struct OrchestrationEngine {
    cross_chain_metrics: CrossChainMetrics,
    transaction_validation: TransactionValidation,
}

impl OrchestrationEngine {
    pub fn new(
        cross_chain_metrics: CrossChainMetrics,
        transaction_validation: TransactionValidation,
    ) -> Self {
        Self {
            cross_chain_metrics,
            transaction_validation,
        }
    }

    pub fn process_cross_chain_transaction(&self) -> (PreciseFloat, bool) {
        let routing_efficiency = self.cross_chain_metrics.calculate_routing_efficiency();
        let (validation_score, is_valid) = self.transaction_validation.validate_transaction();
        
        let final_score = routing_efficiency.mul(&validation_score);
        (final_score, is_valid)
    }
}
