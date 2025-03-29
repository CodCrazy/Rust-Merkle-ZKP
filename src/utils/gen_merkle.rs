use std::{collections::HashMap, sync::Arc};
use anyhow::{Ok, Result}; // For handling errors

use axum::extract::State;
use ff::*;
use poseidon_rs::{Fr, FrRepr, Poseidon};

use crate::{models::ddid_models::CoreIdModel, AppState};

// Assume PoseidonHash is already defined somewhere in your code
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct PoseidonHash([u8; 32]); // Example representation of PoseidonHash

pub struct MerkleTreeStorage {
    leaves: HashMap<Fr, usize>, // Map leaves to their positions
    layers: Vec<Vec<Fr>>,
    capacity: usize,                      // 2^depth maximum leaves
}

pub struct MerkleProof {
    pub siblings: Vec<[u8; 64]>, // Sibling hashes along the path
    pub indice: u32,          // True for right child, False for left child
}

impl MerkleTreeStorage {
    /// Creates a new MerkleTreeStorage with a given depth.
    /// The capacity is 2^depth.
    pub fn new(depth: u32) -> Self {
        let capacity = 1 << (depth - 1); // Calculate 2^depth
        Self {
            leaves: HashMap::new(),
            layers: vec![],
            capacity,
        }
    }

    /// Fetches a MerkleTreeStorage instance from a database pool.
    // pub async fn fetch(pool: &PgPool) -> Self {
    //     // Example: Fetch leaves from the database (adjust for your schema)
    //     let rows = sqlx::query!("SELECT hash, position FROM merkle_leaves")
    //         .fetch_all(pool)
    //         .await
    //         .unwrap();

    //     let mut leaves = HashMap::new();
    //     for row in rows {
    //         let hash = PoseidonHash(row.hash); // Adjust deserialization if needed
    //         leaves.insert(hash, row.position as usize);
    //     }

    //     // Infer depth from the number of leaves or database metadata
    //     let capacity = 1 << (leaves.len().next_power_of_two().trailing_zeros());
    //     Self { leaves, capacity }
    // }

    /// Resets the Merkle tree, clearing all stored leaves.
    pub async fn reset_tree(&mut self) -> Result<()> {
        self.leaves.clear();
        self.layers.clear();
        Ok(())
    }
    pub fn hex_to_fr(&mut self, s: &str) -> Option<Fr>{
        // fn from_str(s: &str) -> Option<Self> {
            if s.is_empty() {
                return None;
            }
    
            if s == "0" {
                return Some(Fr::zero());
            }
    
            let mut res = Fr::zero();
    
            let ten = Fr::from_repr(FrRepr::from(16)).unwrap();
    
            let mut first_digit = true;
    
            for c in s.chars() {
                match c.to_digit(16) {
                    Some(c) => {
                        if first_digit {
                            if c == 0 {
                                continue;
                            }
    
                            first_digit = false;
                        }
    
                        res.mul_assign(&ten);
                        res.add_assign(&Fr::from_repr(FrRepr::from(u64::from(c))).unwrap());
                    }
                    None => {
                        return None;
                    }
                }
            }
    
            Some(res)
        // }
    }
    /// Inserts a new leaf into the tree.
    pub fn insert_leaf(&mut self, leaf: Fr) -> Result<()> {
        if self.leaves.len() >= self.capacity {
            return Err(anyhow::anyhow!("Merkle tree is full"));
        }

        // Use the current number of leaves as the position for the new leaf
        let position = self.leaves.len();
        self.leaves.insert(leaf, position);
        // self.update_merkle_tree(leaf.clone());
        Ok(())
    }

    pub fn insert_leaf_data(&mut self, leaf: Vec<&str>) -> Result<Fr> {
        let leaf_string = leaf.join("");

        let input_bytes = leaf_string.as_bytes();
        
        let leaf_hash = self.hex_to_fr(hex::encode(input_bytes).as_str()).unwrap();
        if self.leaves.len() >= self.capacity {
            return Err(anyhow::anyhow!("Merkle tree is full"));
        }

        // Use the current number of leaves as the position for the new leaf
        let position = self.leaves.len();
        self.leaves.insert(leaf_hash, position);
        // self.update_merkle_tree(leaf.clone());
        Ok(leaf_hash)
    }

    // pub fn generate(hashes: &Vec<Fr>, tree: &mut Vec<Vec<Fr>>) -> Fr{
        // if(hashes.length === 1) {
        //     return hashes;
        // }
        // ensureEven(hashes);
        // const combinedHashes = [];
        // for(let i = 0; i < hashes.length; i += 2) {
        //     const hashesConcatenated = hashes[i] + hashes[i + 1];
        //     const hash = sha256(hashesConcatenated);
        //     combinedHashes.push(hash);
        // }
        // tree.push(combinedHashes);
        // return generate(combinedHashes, tree);
        // let length = hashes.len();
        // if length == 1 {
        //     return hashes[0];
        // }
        // if length % 2 > 0 {
        //     hashes.push(hashes[length-1]);
        // }
        // let combinedHashes:Vec<Fr> = vec![];

        
    // }
    // Cousturct a merkle tree
    pub fn generate_merkle_tree(&mut self) -> Result<()> {
        self.layers.clear();
        let poseidon = Poseidon::new();
        let l: usize = self.leaves.len();
        if self.leaves.len() < 1 {
            return Err(anyhow::anyhow!("There are not any leaves"))
        }
        let mut ordered_leaves: Vec<Fr> = vec![Fr::from_str("0").unwrap(); l];
        for leaf in self.leaves.clone() {
            ordered_leaves[leaf.1] = leaf.0;
        }
        self.layers.push(ordered_leaves.clone());

        let mut current_layer = ordered_leaves.clone();
        
        while current_layer.len() > 1 {
            let mut next_layer: Vec<Fr> = Vec::new();
            for i in (0..current_layer.len()).step_by(2) {
                let left = &current_layer[i];
                let right = if i + 1 < current_layer.len() {
                    &current_layer[i + 1]
                } else {
                    left
                };
                next_layer.push(poseidon.hash(vec![*left, *right]).unwrap());
            }
            current_layer = next_layer;
            self.layers.push(current_layer.clone());
        }

        Ok(())
    }

    pub fn update_merkle_tree(&mut self, input_updated_hash: Fr) -> Result<()> {
        // self.layers.clear();
        let poseidon = Poseidon::new();
        let l: usize = self.leaves.len();
        if self.leaves.len() < 1 {
            return Err(anyhow::anyhow!("There are not any leaves"))
        }
        let mut ordered_leaves: Vec<Fr> = vec![Fr::from_str("0").unwrap(); l];
        for leaf in self.leaves.clone() {
            ordered_leaves[leaf.1] = leaf.0;
        }
        let mut updated_pos = *self.leaves.get(&input_updated_hash).expect("Hash not exist");
        let mut updated_hash = input_updated_hash.clone();
        // for i in 0..self.layers.len() {
        let mut i: usize = 0;
        loop {
            if i == self.layers.len() {
                self.layers.push(vec![updated_hash.clone()]);
                break;
            } else if i == self.layers.len() - 1 && updated_pos == 0 {
                self.layers[i] = vec![updated_hash.clone()];
                break;
            }
            if updated_pos == self.layers[i].len(){
                self.layers[i].push(updated_hash.clone());
            } else {
                self.layers[i][updated_pos] = updated_hash.clone();
            }
            let sibling = if updated_pos % 2 == 0 {
                if updated_pos == self.layers[i].len() - 1 {
                    &self.layers[i][updated_pos]
                } else {
                    &self.layers[i][updated_pos + 1]
                }
            } else {
                &self.layers[i][updated_pos - 1]
            };
            updated_pos /= 2;
            updated_hash = poseidon.hash(vec![updated_hash.clone(), sibling.clone()]).unwrap();
            i += 1;
        }
        Ok(())
    }

    /// Generates a Merkle proof for a given leaf.
    pub fn generate_merkle_proof(&self, leaf: Fr) -> Option<MerkleProof> {
        let position = self.leaves.get(&leaf)?;
        let mut siblings = Vec::new();
        let mut indices = Vec::new();
        let mut index = *position;

        // Simulate a binary tree structure for proof generation
        for i in 0..self.layers.len() {
            let sibling = if index % 2 == 0 {
                if index == self.layers[i].len() - 1 {
                    self.layers[i][index].clone()
                } else {
                    self.layers[i][index + 1].clone()
                }
            } else {
                self.layers[i][index - 1].clone()
            };
            if index % 2 == 0 {
                indices.push(true);
            } else {
                indices.push(false);
            }
            siblings.push(to_hex(&sibling).as_bytes().try_into().unwrap());
            index /= 2;
        }
        let mut indice = 0;

        for &bit in &indices {
            indice = (indice << 1) | (bit as u32);
        }
        Some(MerkleProof { siblings, indice })
    }
}

pub fn merkle_proof_callback(State(data): State<Arc<AppState>>, target_leaf_data: CoreIdModel) -> Result<(MerkleProof, [u8; 64]), anyhow::Error> {    
    
    let mut target_leaf_vec = Vec::new();
    target_leaf_vec.push(target_leaf_data.embedding_hash.as_str());
    target_leaf_vec.push(target_leaf_data.name.as_str());
    target_leaf_vec.push(target_leaf_data.breed.as_str());
    let formatted_time = target_leaf_data.date_of_birth.format("%Y-%m-%d").to_string();
    target_leaf_vec.push(formatted_time.as_str());
    let proof_level_str = target_leaf_data.proof_level.to_string();
    target_leaf_vec.push(proof_level_str.as_str());

    let target_leaf_hash = data.merkle_tree.write().unwrap().insert_leaf_data(target_leaf_vec.clone()).unwrap();
    let _ = data.merkle_tree.write().unwrap().update_merkle_tree(target_leaf_hash);
    let target_leaf_hash_out = to_hex(&target_leaf_hash).as_bytes().try_into().unwrap();
    // Attempt to generate a Merkle proof for a leaf
    if let Some(proof) = data.merkle_tree.write().unwrap().generate_merkle_proof(target_leaf_hash) {
        println!("Generated Merkle proof for leaf {:?}:", target_leaf_hash);
        println!("  Siblings: {:?}", proof.siblings);
        println!("  Indices: {:?}", proof.indice);
        Ok((proof, target_leaf_hash_out))
    } else {
        println!("Leaf {:?} not found in the tree", target_leaf_hash);
        Err(anyhow::anyhow!("Leaf Not Found"))
    }
}