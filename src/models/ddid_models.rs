
// use crate::macros::merkle_tree_macro::MerkleTree;
// use chrono::NaiveDate as Date;
// use crate::storage::merkle_tree_storage::MerkleTreeStorage;
// use crate::traits::merkle_tree::MerkleTree;
// use crate::utils::hashing::PoseidonHash;

// #[derive(sqlx::FromRow, MerkleTree)]
// #[tree(depth = 32, storage = "CoreIdTree")]
// pub struct CoreId {
//     pub embedding_hash: String,
//     // Other fields...
// }

// impl CoreId {
//     pub fn new_example() -> Self {
//         Self {
//             embedding_hash: "example_hash".to_string(),
//             // Initialize other fields...
//         }
//     }
// }

// #[async_trait::async_trait]
// impl MerkleTree for CoreId {
//     type Storage = MerkleTreeStorage;

//     const CONTRACT_ADDRESS: [u8; 32] = [0; 32];
//     const DEPTH: u32 = 32;

//     fn to_leaf_hash(&self) -> PoseidonHash {
//         PoseidonHash::hash(self.embedding_hash.as_bytes())
//     }

//     fn tree_storage(&self) -> MerkleTreeStorage {
//         MerkleTreeStorage::new(Self::DEPTH)
//     }

//     async fn read_on_chain_root(&self) -> PoseidonHash {
//         PoseidonHash::default()
//     }
// }


use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CoreIdModel {
    pub id: i32,
    pub embedding_hash: String,
    pub name: String,
    pub breed: String,
    pub date_of_birth: chrono::NaiveDate,
    pub proof_level: i16,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}


#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MerchantJoinIdModel {
    pub id: i32,
	pub merchant_id: i32,
	pub embedding_hash: String,
	pub write_fields: Vec<String>,
	pub read_merchant_fields: Value, // can read fields of other merchants
	pub last_updated: chrono::DateTime<chrono::Utc>,
	pub latest_data_hash: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct MerchantRecordModel {
    pub id: i32,
	pub embedding_hash: String,
	pub merchant_id: u32,
	pub date_issued: chrono::DateTime<chrono::Utc>,
	pub valid_until: Option<chrono::DateTime<chrono::Utc>>,
	pub prev_data_hash: String, // should match latest_data_hash pre-update
	pub data_record: Value,
	pub data_hash: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct CoreIdTree {
    pub leaves: Value,
	pub capacity: i32,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct MerchantJoinTree {
    pub leaves: Value,
	pub capacity: i64,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct MerchantRecordTree {
    pub leaves: Value,
	pub capacity: i64,
}
