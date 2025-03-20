use std::collections::HashMap;
use anyhow::Result; // For handling errors
use sqlx::PgPool;   // For database connection
use async_trait::async_trait; // For async functions with traits

// use rand::rngs::StdRng;
// use rand::SeedableRng;
// use ff::Field;
use ff::*;

use poseidon_rs::{Fr, Poseidon};

// Assume PoseidonHash is already defined somewhere in your code
#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct PoseidonHash([u8; 32]); // Example representation of PoseidonHash

pub struct MerkleTreeStorage {
    leaves: HashMap<Fr, usize>, // Map leaves to their positions
    layers: Vec<Vec<Fr>>,
    capacity: usize,                      // 2^depth maximum leaves
}

pub struct MerkleProof {
    pub siblings: Vec<Fr>, // Sibling hashes along the path
    pub indices: Vec<bool>,          // True for right child, False for left child
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

    pub fn update_merkle_tree(&mut self, updatedHash: Fr) -> Result<()> {
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
        let mut updated_pos = *self.leaves.get(&updatedHash).expect("Hash not exist");
        let mut updated_hash = updatedHash.clone();
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
            siblings.push(sibling);
            index /= 2;
        }

        Some(MerkleProof { siblings, indices })
    }
}

fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // Create a MerkleTreeStorage with a depth of 3 (capacity = 2^3 = 8 leaves)
    let mut tree = MerkleTreeStorage::new(20);
    println!("Initialized Merkle Tree with capacity: {}", tree.capacity);

    // Add some leaves
    let leaves = vec![
        Fr::from_str("0").unwrap(),
        Fr::from_str("1").unwrap(),
        Fr::from_str("2").unwrap(),
        Fr::from_str("3").unwrap(),
        Fr::from_str("4").unwrap(),
        Fr::from_str("5").unwrap(),
        Fr::from_str("6").unwrap(),
        Fr::from_str("7").unwrap(),
    ];

    for (i, leaf) in leaves.iter().enumerate() {
        match tree.insert_leaf(*leaf) {
            Ok(_) => println!("Inserted leaf {}: {:?}", i, leaf),
            Err(e) => println!("Failed to insert leaf {}: {:?}", i, e),
        }
    }
    let _ = tree.generate_merkle_tree();
    // Attempt to generate a Merkle proof for a leaf
    let target_leaf = Fr::from_str("1").unwrap();
    if let Some(proof) = tree.generate_merkle_proof(target_leaf) {
        println!("Generated Merkle proof for leaf {:?}:", target_leaf);
        println!("  Siblings: {:?}", proof.siblings);
        println!("  Indices: {:?}", proof.indices);
    } else {
        println!("Leaf {:?} not found in the tree", target_leaf);
    }

    // Test inserting beyond capacity
    let extra_leaf = Fr::from_str("8").unwrap();
    match tree.insert_leaf(extra_leaf) {
        Ok(_) => println!("Inserted extra leaf: {:?}", extra_leaf),
        Err(e) => println!("Failed to insert extra leaf: {:?}", e),
    }

    let _ = tree.update_merkle_tree(extra_leaf);
    let target_leaf = Fr::from_str("8").unwrap();
    if let Some(proof) = tree.generate_merkle_proof(target_leaf) {
        println!("Generated Merkle proof for leaf {:?}:", target_leaf);
        println!("  Siblings: {:?}", proof.siblings);
        println!("  Indices: {:?}", proof.indices);
    } else {
        println!("Leaf {:?} not found in the tree", target_leaf);
    }
    
    // Reset the tree and show its state
    if let Err(err) = tree.reset_tree().await {
        eprintln!("Failed to reset the tree: {:?}", err);
    } else {
        println!("Tree reset successfully!");
    }
    println!("Tree reset. Current leaf count: {}", tree.leaves.len());
    });
    
}