use crate::storage::merkle_tree_storage::MerkleTreeStorage;
use crate::utils::hashing::PoseidonHash;

#[async_trait::async_trait]
pub trait MerkleTree {
    type Storage;

    const CONTRACT_ADDRESS: [u8; 32];
    const DEPTH: u32;

    fn to_leaf_hash(&self) -> PoseidonHash;
    fn tree_storage(&self) -> MerkleTreeStorage;
    async fn read_on_chain_root(&self) -> PoseidonHash;
}
