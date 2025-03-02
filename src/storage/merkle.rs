use sha2::{Sha256, Digest};

pub struct MerkleTree {
    pub root: Vec<u8>,
    pub leaves: Vec<Vec<u8>>
}

impl MerkleTree {
    pub fn new() -> Self {
        Self {
            root: vec![],
            leaves: vec![]
        }
    }

    pub fn add_leaf(&mut self, data: &[u8]) {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hasher.finalize().to_vec();
        self.leaves.push(hash);
        self.update_root();
    }

    fn update_root(&mut self) {
        if self.leaves.is_empty() {
            self.root = vec![];
            return;
        }

        let mut current = self.leaves.clone();
        while current.len() > 1 {
            let mut next = Vec::new();
            for chunk in current.chunks(2) {
                let mut hasher = Sha256::new();
                hasher.update(&chunk[0]);
                if chunk.len() > 1 {
                    hasher.update(&chunk[1]);
                }
                next.push(hasher.finalize().to_vec());
            }
            current = next;
        }
        self.root = current[0].clone();
    }
}
