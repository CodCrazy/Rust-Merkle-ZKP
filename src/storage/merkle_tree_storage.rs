use crate::utils::hashing::PoseidonHash;
use std::collections::HashMap;

pub struct MerkleTreeStorage {
    pub leaves: HashMap<PoseidonHash, usize>,
    pub capacity: usize,
}

impl MerkleTreeStorage {
    pub fn new(depth: u32) -> Self {
        Self {
            leaves: HashMap::new(),
            capacity: 2_usize.pow(depth),
        }
    }

    pub fn insert_leaf(&mut self, leaf: PoseidonHash) -> Result<(), String> {
        if self.leaves.len() >= self.capacity {
            return Err("Tree is at full capacity".into());
        }
        self.leaves.insert(leaf, self.leaves.len());
        Ok(())
    }

    pub fn generate_merkle_proof(&self, leaf: PoseidonHash) -> Option<MerkleProof> {
        // Generate and return Merkle proof
        Some(MerkleProof {
            siblings: vec![],
            indices: vec![],
        })
    }
}

pub struct MerkleProof {
    pub siblings: Vec<PoseidonHash>,
    pub indices: Vec<bool>,
}
