use crate::math::precision::PreciseFloat;

pub struct HubbleState {
    pub total_supply: PreciseFloat,
    pub circulating_supply: PreciseFloat,
    pub staked_supply: PreciseFloat,
    pub validator_count: u64
}

impl HubbleState {
    pub fn new() -> Self {
        Self {
            total_supply: PreciseFloat::new(0, 0),
            circulating_supply: PreciseFloat::new(0, 0),
            staked_supply: PreciseFloat::new(0, 0),
            validator_count: 0
        }
    }
}
