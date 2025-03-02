use serde::{Serialize, Deserialize};
use crate::math::precision::PreciseFloat;

#[derive(Debug, Serialize, Deserialize)]
pub struct Contract {
    pub address: [u8; 32],
    pub code: Vec<u8>,
    pub state: ContractState
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractState {
    pub balance: PreciseFloat,
    pub storage: Vec<u8>,
    pub nonce: u64
}

impl Contract {
    pub fn new(address: [u8; 32], code: Vec<u8>) -> Self {
        Self {
            address,
            code,
            state: ContractState {
                balance: PreciseFloat::new(0, 0),
                storage: Vec::new(),
                nonce: 0
            }
        }
    }
}
