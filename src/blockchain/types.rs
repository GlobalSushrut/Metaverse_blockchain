use std::borrow::Borrow;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct QuantumNodeID([u8; 32]);

impl QuantumNodeID {
    pub fn new(id: [u8; 32]) -> Self {
        Self(id)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl Borrow<[u8; 32]> for QuantumNodeID {
    fn borrow(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<[u8; 32]> for QuantumNodeID {
    fn from(id: [u8; 32]) -> Self {
        Self(id)
    }
}
