use std::collections::HashMap;
use crate::math::precision::PreciseFloat;

pub struct VMState {
    pub memory: Vec<u8>,
    pub registers: HashMap<String, PreciseFloat>,
    pub program_counter: usize
}

impl VMState {
    pub fn new() -> Self {
        Self {
            memory: Vec::new(),
            registers: HashMap::new(),
            program_counter: 0
        }
    }

    pub fn reset(&mut self) {
        self.memory.clear();
        self.registers.clear();
        self.program_counter = 0;
    }
}
