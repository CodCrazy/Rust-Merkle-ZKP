use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoreIdSchema {
    // pub image_uris: HashMap<String, String>,
    pub embedding_hash: String,
    pub name: String,
    pub breed: String,
    pub dob: String,
    pub proof_level: i32,
    pub microchip_id: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IsDdidMemberSchema {
    pub leaf_hash: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AddMerchantSchema {
    pub merchant_id: i32,
    pub leaf_hash: String,
    pub read_fields: BTreeMap<i32, Vec<String>>,
    pub write_permission: bool
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMerchantSchema {
    pub last_data_hash: Option<String>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct WriteMerchantRecordSchema {
    pub id: uuid::Uuid,
    pub merchant_id: i32,
    pub leaf_hash: String,
    pub data_record: Value, 
    pub prev_record_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReadMerchantRecordSchema {
    pub merchant_id: i32,
    pub embedding_hash: String,
    pub requested_fields: Vec< String>,
}
