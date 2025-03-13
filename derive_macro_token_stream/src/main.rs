use std::collections::HashMap;
use anyhow::Result; // For handling errors
use sqlx::PgPool;   // For database connection
use async_trait::async_trait; // For async functions with traits

// use rand::rngs::StdRng;
// use rand::SeedableRng;
// use ff::Field;
use ff::*;

use poseidon_rs::{Fr, Poseidon};

use merkle_tree_storage::MerkleTreeStorage;
// use merkle_tree::MerkleTree;


fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        // Create a MerkleTreeStorage with a depth of 3 (capacity = 2^3 = 8 leaves)
    let mut tree = MerkleTreeStorage::new(20);
    // println!("Initialized Merkle Tree with capacity: {}", tree.capacity);

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
    // println!("Tree reset. Current leaf count: {}", tree.leaves.len());
    });
    
}