use std::collections::{HashMap, BTreeMap};
use chrono::{DateTime, Date};
use anyhow::Result; // For handling errors
use sqlx::PgPool;   // For database connection
use async_trait::async_trait; // For async functions with traits

// use rand::rngs::StdRng;
// use rand::SeedableRng;
// use ff::Field;
use ff::*;

use poseidon_rs::{Fr, Poseidon};
// mod merkle_tree;
// mod merkle_tree_storage;
// use crate::merkle_tree::MerkleTree;
// use merkle_tree_storage::MerkleTreeStorage;
use tree_proc_macros::MerkleTree;
mod merkle_tree;
use merkle_tree::MerkleTree;
use merkle_tree_storage::MerkleTreeStorage;


type PoseidonHash = Fr;


#[derive(MerkleTree, sqlx::FromRow)]
#[tree(depth = 32, storage = "CoreIdTree")]
pub struct CoreId {
	#[tree_arg]
	embedding_hash: String,
	merchants: Vec<MerchantJoinId>,
	records: Vec<MerchantRecord>,
	#[tree_arg]
	name: String,
	#[tree_arg]
	breed: String,
	#[tree_arg]
	date_of_birth: Date,
	#[tree_arg]
	proof_level: u8,
	#[tree_arg]
	microchip_id: String,
}

#[derive(MerkleTree, sqlx::FromRow)]
#[tree(depth = 32, storage = "MerchantJoinTree")]
pub struct MerchantJoinId {
	#[tree_arg]
	merchant_id: i32,
	#[tree_arg]
	embedding_hash: String,
	write_fields: Vec<String>,
	read_merchant_fields: Vec<BTreeMap<u32, Vec<String>>>, // can read fields of other merchants
	#[tree_arg]
	last_updated: DateTime,
	#[tree_arg]
	latest_data_hash: PoseidonHash
}

#[derive(MerkleTree, sqlx::FromRow)]
#[tree(depth = 32, storage = "MerchantRecordTree")]
pub struct MerchantRecord {
	#[tree_arg]
	embedding_hash: String,
	#[tree_arg]
	merchant_id: u32,
	#[tree_arg]
	date_issued: DateTime,
	#[tree_arg]
	valid_until: Optional<DateTime>,
	#[tree_arg]
	prev_data_hash: PoseidonHash, // should match latest_data_hash pre-update
	data_record: Value,
	#[tree_arg]
	data_hash: PoseidonHash, // should be the hash of prevDataHash & dataRecord
}

#[derive(sqlx::FromRow)]
pub struct MerchantData {
	merchant_id: i32,
	schema: Value,
	readable_fields: Vec<String>,
}

// pub struct MerkleTreeStorage {
// 	leaves: HashMap<PoseidonHash, usize>,
// 	capacity: usize, // 2^depth maximum leaves
// }

#[derive(sqlx::FromRow)]
pub struct CoreIdTree(MerkleTreeStorage);
#[derive(sqlx::FromRow)]
pub struct MerchantJoinTree(MerkleTreeStorage);
#[derive(sqlx::FromRow)]
pub struct MerchantRecordTree(MerkleTreeStorage);
