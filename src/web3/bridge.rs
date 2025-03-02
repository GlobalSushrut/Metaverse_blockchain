use serde::{Serialize, Deserialize};
use crate::math::precision::PreciseFloat;

#[derive(Debug, Serialize, Deserialize)]
pub struct Bridge {
    pub source_chain: [u8; 32],
    pub target_chain: [u8; 32],
    pub locked_assets: PreciseFloat,
    pub validators: Vec<[u8; 32]>
}

impl Bridge {
    pub fn new(source: [u8; 32], target: [u8; 32]) -> Self {
        Self {
            source_chain: source,
            target_chain: target,
            locked_assets: PreciseFloat::new(0, 0),
            validators: Vec::new()
        }
    }

    pub fn validate_transfer(&self, _amount: PreciseFloat) -> bool {
        // Implementation will go here
        true
    }
}
