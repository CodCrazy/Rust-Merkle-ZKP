use merkle_tree_storage::MerkleTreeStorage;
use poseidon_rs::{Fr, Poseidon};

pub trait MerkleTree{
    type Storage;
    const CONTRACT_ADDRESS: [u8; 32];
    const DEPTH: u32;

    fn to_leaf_hash(&self) -> Fr;
    fn tree_storage(&self) -> MerkleTreeStorage;
    async fn read_on_chain_root(&self) -> Fr;
}