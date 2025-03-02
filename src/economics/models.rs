use crate::math::precision::PreciseFloat;
use std::collections::HashMap;

/// Economic Modeling System
pub struct EconomicModel {
    precision: u8,
    parameters: ModelParameters,
    state: SystemState,
    history: Vec<StateSnapshot>,
    validators: HashMap<ValidatorId, ValidatorState>,
}

type ValidatorId = [u8; 32];

#[derive(Clone)]
struct ModelParameters {
    inflation_rate: PreciseFloat,
    transaction_fee_rate: PreciseFloat,
    validator_reward_rate: PreciseFloat,
    stake_lockup_period: u64,
    minimum_stake: PreciseFloat,
    maximum_stake: PreciseFloat,
}

#[derive(Clone)]
struct SystemState {
    total_supply: PreciseFloat,
    circulating_supply: PreciseFloat,
    total_staked: PreciseFloat,
    total_transactions: u64,
    average_fee: PreciseFloat,
    network_utilization: PreciseFloat,
}

#[derive(Clone)]
struct ValidatorState {
    stake: PreciseFloat,
    rewards: PreciseFloat,
    performance_score: PreciseFloat,
    last_active: u64,
    total_validated: u64,
}

#[derive(Clone)]
struct StateSnapshot {
    state: SystemState,
    timestamp: u64,
    metrics: HashMap<String, PreciseFloat>,
}

impl EconomicModel {
    pub fn new(precision: u8) -> Self {
        Self {
            precision,
            parameters: ModelParameters {
                inflation_rate: PreciseFloat::new(200, 2), // 2.00% annual
                transaction_fee_rate: PreciseFloat::new(10, 2), // 0.10%
                validator_reward_rate: PreciseFloat::new(500, 2), // 5.00% annual
                stake_lockup_period: 14 * 24 * 60 * 60, // 14 days in seconds
                minimum_stake: PreciseFloat::new(100000, 2), // 1000.00 tokens
                maximum_stake: PreciseFloat::new(1000000000, 2), // 10000000.00 tokens
            },
            state: SystemState {
                total_supply: PreciseFloat::new(1000000000000, 2), // 10B initial supply
                circulating_supply: PreciseFloat::new(700000000000, 2), // 7B circulating
                total_staked: PreciseFloat::new(300000000000, 2), // 3B staked
                total_transactions: 0,
                average_fee: PreciseFloat::new(10, 2), // 0.10 tokens
                network_utilization: PreciseFloat::new(0, 2),
            },
            history: Vec::new(),
            validators: HashMap::new(),
        }
    }

    pub fn calculate_inflation(&self) -> PreciseFloat {
        // Calculate inflation based on network metrics
        let base_inflation = self.parameters.inflation_rate
            .div(&PreciseFloat::new(100, 2)); // Convert to decimal

        let utilization_factor = self.state.network_utilization
            .div(&PreciseFloat::new(100, 2))
            .mul(&PreciseFloat::new(50, 2)); // Max 0.50% adjustment

        let stake_ratio = self.state.total_staked
            .div(&self.state.total_supply);
        
        let stake_factor = PreciseFloat::new(100, 2)
            .sub(&stake_ratio.mul(&PreciseFloat::new(100, 2)))
            .div(&PreciseFloat::new(100, 2))
            .mul(&PreciseFloat::new(50, 2)); // Max 0.50% adjustment

        base_inflation
            .add(&utilization_factor)
            .add(&stake_factor)
    }

    pub fn calculate_validator_rewards(
        &self,
        validator_id: &ValidatorId
    ) -> Result<PreciseFloat, &'static str> {
        let validator = self.validators.get(validator_id)
            .ok_or("Validator not found")?;

        // Calculate base rewards
        let base_reward = validator.stake
            .mul(&self.parameters.validator_reward_rate)
            .div(&PreciseFloat::new(100, 2)); // Convert to decimal

        // Apply performance multiplier
        let performance_multiplier = validator.performance_score
            .div(&PreciseFloat::new(100, 2));

        // Calculate final reward
        Ok(base_reward.mul(&performance_multiplier))
    }

    pub fn update_network_metrics(
        &mut self,
        transactions: u64,
        fees: PreciseFloat,
        utilization: PreciseFloat
    ) {
        // Update state
        self.state.total_transactions += transactions;
        let current_fee = self.state.average_fee.clone();
        self.state.average_fee = self.calculate_moving_average(
            current_fee,
            fees,
            PreciseFloat::new(10, 2) // 0.10 weight for new value
        );
        self.state.network_utilization = utilization;

        // Create snapshot
        self.record_snapshot();
    }

    pub fn stake_tokens(
        &mut self,
        validator_id: ValidatorId,
        amount: PreciseFloat
    ) -> Result<(), &'static str> {
        // Validate stake amount
        if amount.value < self.parameters.minimum_stake.value {
            return Err("Stake amount below minimum");
        }
        if amount.value > self.parameters.maximum_stake.value {
            return Err("Stake amount above maximum");
        }

        // Update validator state
        let validator = self.validators.entry(validator_id)
            .or_insert(ValidatorState {
                stake: PreciseFloat::new(0, self.precision),
                rewards: PreciseFloat::new(0, self.precision),
                performance_score: PreciseFloat::new(100, 2), // Initial 1.00 score
                last_active: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                total_validated: 0,
            });

        // Update stakes
        validator.stake = validator.stake.add(&amount);
        self.state.total_staked = self.state.total_staked.add(&amount);
        self.state.circulating_supply = self.state.circulating_supply.sub(&amount);

        Ok(())
    }

    pub fn calculate_transaction_fee(
        &self,
        transaction_size: u64,
        priority: PreciseFloat
    ) -> PreciseFloat {
        // Calculate base fee
        let base_fee = PreciseFloat::new(transaction_size as i128, 0)
            .mul(&self.parameters.transaction_fee_rate)
            .div(&PreciseFloat::new(100, 2));

        // Apply network utilization multiplier
        let utilization_multiplier = PreciseFloat::new(100, 2)
            .add(&self.state.network_utilization)
            .div(&PreciseFloat::new(100, 2));

        // Apply priority multiplier
        let priority_multiplier = priority
            .div(&PreciseFloat::new(100, 2))
            .add(&PreciseFloat::new(100, 2))
            .div(&PreciseFloat::new(100, 2));

        base_fee
            .mul(&utilization_multiplier)
            .mul(&priority_multiplier)
    }

    fn calculate_moving_average(
        &self,
        current: PreciseFloat,
        new_value: PreciseFloat,
        weight: PreciseFloat
    ) -> PreciseFloat {
        let inverse_weight = PreciseFloat::new(100, 2).sub(&weight);
        current.mul(&inverse_weight)
            .add(&new_value.mul(&weight))
            .div(&PreciseFloat::new(100, 2))
    }

    fn record_snapshot(&mut self) {
        let snapshot = StateSnapshot {
            state: self.state.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            metrics: self.calculate_metrics(),
        };

        self.history.push(snapshot);

        // Keep last 1000 snapshots
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    fn calculate_metrics(&self) -> HashMap<String, PreciseFloat> {
        let mut metrics = HashMap::new();

        // Calculate key metrics
        metrics.insert(
            "stake_ratio".to_string(),
            self.state.total_staked.div(&self.state.total_supply)
        );

        metrics.insert(
            "transaction_velocity".to_string(),
            PreciseFloat::new(
                self.state.total_transactions as i128,
                0
            ).div(&self.state.circulating_supply)
        );

        metrics.insert(
            "network_efficiency".to_string(),
            self.state.network_utilization.mul(&self.state.average_fee)
        );

        metrics
    }
}
