use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::{Utc, Date, NaiveDate};

// use poseidon_rs::{Fr, Poseidon};
// use serde_json::{from_value, json};
// use sqlx::Error;

use crate::{
    models::ddid_models::*, schemas::ddid_schemas::*, utils::{gen_merkle::{merkle_proof_callback, MerkleTreeStorage}, gen_zkp::insert_leaf_zkp, get_current_leaves::get_current_leaves, get_onchain_root::get_current_root, ml_model::ml_model}, AppState
};

pub async fn prove_ddid_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CoreIdSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    // let embedding_hash = match ml_model(body.image_uris) {
    //     Ok(hash) => hash, 
    //     Err(_) => {
    //         let proof_json = serde_json::json!({
    //             "success": false,
    //             "proof_response": None,
    //             "error": Some("IMAGE_ERROR".to_string()),
    //         });
    //         return Err((StatusCode::NOT_ACCEPTABLE, Json(proof_json)));
    //     }
    // };
    let embedding_hash_query_result = sqlx::query_as::<_, CoreIdModel>(
        r#"SELECT * FROM coreid WHERE embedding_hash = $1"#
    )
    .bind(body.embedding_hash.clone())
    .fetch_all(&data.db)
    .await;
    if embedding_hash_query_result.unwrap().len() > 0 {
        let _update_proof_level_query_result = sqlx::query_as::<_, CoreIdModel>(
            r#"UPDATE coreid SET proof_level = $2  WHERE embedding_hash = $1"#
        )
        .bind(body.embedding_hash)
        .bind(body.proof_level)
        .fetch_one(&data.db)
        .await;
        let proof_json = serde_json::json!({
            "success" : true,
            "proof_response" : "valid_proof"
        });
        Ok(Json(proof_json))
    } else {
        // New embedding_hash: assert all parameters are provided
        if body.name.is_empty() || body.breed.is_empty() || body.dob.is_empty() {
            let proof_json = serde_json::json!({
                "success": false,
                "proof_response": "invalid_proof",
                "error": Some("MISSING_PARAMS".to_string()),
            });
            return Err((StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, Json(proof_json)));
        }
        let date_of_birth = NaiveDate::parse_from_str(&body.dob, "%Y-%m-%d").unwrap();
        let insert_query_result = sqlx::query_as::<_, CoreIdModel>(
            r#"INSERT into coreid (embedding_hash, name, breed, date_of_birth, proof_level, microchip_id) VALUES ($1, $2, $3, $4, $5, $6)"#
        )
        .bind(body.embedding_hash)
        .bind(body.name)
        .bind(body.breed)
        .bind(date_of_birth)
        .bind(body.proof_level)
        .bind(body.microchip_id)
        .fetch_one(&data.db)
        .await;
        
        let (merkle_proof, target_leaf) = merkle_proof_callback(State(data), insert_query_result.unwrap()).unwrap();
        let zkp = insert_leaf_zkp(target_leaf, merkle_proof).await;
        if zkp {
            let proof_json = serde_json::json!({
                "success" : true,
                "proof_response" : "valid_proof",
            });
            Ok(Json(proof_json))
        } else {
            let proof_json = serde_json::json!({
                "success" : false,
                "proof_response" : "invalid_proof",
                "error": Some("VERIFICATION_FAILED".to_string()),
            });
            return Err((StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, Json(proof_json)));
        }
        
    }
}


// pub async fn is_ddid_member_handler(
//     Json(body): Json<IsDdidMemberSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
// }

// pub async fn add_merchant_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<AddMerchantSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
// }

// pub async fn write_merchant_record_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<WriteMerchantRecordSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    
// }

// pub async fn read_merchant_record_handler(
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<ReadMerchantRecordSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    
// }
