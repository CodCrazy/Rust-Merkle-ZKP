use ark_circom::{CircomConfig, CircomBuilder};
use ark_bn254::{Bn254, Fr, G1Projective};
use ark_std::log2;
use rand::SeedableRng;
use rand::rngs::StdRng;
use ark_groth16::{prepare_verifying_key, Groth16};
use ark_snark::SNARK;
use ark_serialize::{CanonicalSerialize, Compress};
use num_bigint::BigInt;
use borsh::{to_vec, BorshDeserialize, BorshSerialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::instruction::{AccountMeta, Instruction};
use solana_program::pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::signer::EncodableKey;
use solana_sdk::transaction::Transaction;
use std::str::FromStr;



use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::connect_async;
use url::Url;
use serde_json::Value;

use tokio::{sync::oneshot, task};

use super::prove::setup;
use super::verify_lite::{build_verifier, Groth16VerifierPrepared};
use super::gen_merkle::MerkleProof;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramInstruction {
    VerifyProof(Groth16VerifierPrepared),
}


// pub fn fr_to_hex(fr:Fr) -> [u8; 64] {
//     // Manually extract the internal representation (assumes Fr is represented in 4 limbs of u64)
//     let limbs: [u64; 4] = unsafe { std::mem::transmute(fr) };
    
//     // Convert the 4 limbs into bytes (big-endian representation)
//     let mut bytes = [0u8; 32];
//     for (i, &limb) in limbs.iter().enumerate() {
//         bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_be_bytes());
//     }

//     // Convert bytes to hex string
//     let mut hex_out = [0u8; 64];
//     for (i, byte) in bytes.iter().enumerate() {
//         let hex = format!("{:02x}", byte);
//         hex_out[i * 2..i * 2 + 2].copy_from_slice(hex.as_bytes());
//     }

//     hex_out
// }

async fn event_listener(rx: oneshot::Receiver<()>) -> String {
    let ws_url = "wss://api.devnet.solana.com";
    let (ws_stream, _) = connect_async(Url::parse(ws_url).unwrap().to_string())
        .await
        .expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();

    let program_pubkey = "EjmMQEjv222Mz7u8jUQPC5aJ1pGDEh7xTFTupkELYV3v";
    let subscription_msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "logsSubscribe",
        "params": [
            {"mentions": [program_pubkey]},
            {"commitment": "confirmed"}
        ]
    });

    write.send(subscription_msg.to_string().into()).await.unwrap();
    println!("Subscribed to logs for program: {}", program_pubkey);

    // Wait for the signal before processing logs
    rx.await.expect("Failed to receive signal");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(msg) => {
                if let Ok(text) = msg.into_text() {
                    if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                        if let Some(logs) = parsed["params"]["result"]["value"]["logs"].as_array() {
                            if logs.len() > 3 {
                                if let Some(log) = logs[3].as_str() {
                                    if let Some(stripped_log) = log.strip_prefix("Program log: ") {
                                        println!("Extracted Log: {}", stripped_log);
                                        return stripped_log.to_string(); // Return extracted log
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }
    "No log found".to_string() // Default return value

}




async fn zkp_verification(tx: oneshot::Sender<()>, target_leaf: [u8; 64], merkle_proof: MerkleProof) {
    // Load the WASM and R1CS for witness and proof generation
    let cfg = CircomConfig::<Fr>::new(
        "../../build/circuits/InsertLeaf_js/InsertLeaf.wasm",
        "../../build/circuits/InsertLeaf.r1cs",
    ).unwrap();
    
    let mut builder = CircomBuilder::new(cfg);

    // Use Fr::from_str for large field elements
    let new_root = merkle_proof.siblings[merkle_proof.siblings.len() - 1];
    let new_leaf = target_leaf;
    let path_indices = merkle_proof.indice; // 11101 in binary
    let depth = log2(path_indices as usize);
    // Push inputs to the builder
    builder.push_input("newLeaf", BigInt::parse_bytes(&new_leaf, 16).unwrap());
    builder.push_input("newRoot", BigInt::parse_bytes(&new_root, 16).unwrap());
    builder.push_input("pathIndices", path_indices);
    builder.push_input("depth", depth);

    // Push the Poseidon hash values for pathElements
    for i in 0..merkle_proof.siblings.len()-1 {
        let hash_bytes = merkle_proof.siblings[i];
        builder.push_input("pathElements", BigInt::parse_bytes(&hash_bytes, 16).unwrap());
    }
    for _i in 0..(21-merkle_proof.siblings.len()) {
        builder.push_input("pathElements", BigInt::parse_bytes(b"0000000000000000000000000000000000000000000000000000000000000000", 16).unwrap());
    }

    // Create an empty instance for setup
    let circom = builder.setup();

    // Run the trusted setup
    let mut rng = StdRng::from_entropy();
    let (proving_key, verifying_key) = setup(false, circom.clone());
    let params = Groth16::<Bn254>::generate_random_parameters_with_reduction(circom, &mut rng).unwrap();
    println!("Verifying Key: {:?}", params.vk);
    // Build the witness
    let circom = builder.build().unwrap();

    let public_inputs_fr  = circom.get_public_inputs().unwrap();

    // Create a proof
    let proof = Groth16::<Bn254>::prove(&proving_key, circom.clone(), &mut rng).unwrap();
    let mut proof_bytes = Vec::with_capacity(proof.serialized_size(Compress::No));
    proof
        .serialize_uncompressed(&mut proof_bytes)
        .expect("Error serializing proof");

    let prepared_verifying_key = prepare_verifying_key(&verifying_key);

    let public_inputs: G1Projective =
        Groth16::<Bn254>::prepare_inputs(&prepared_verifying_key, &public_inputs_fr)
            .expect("Error preparing inputs with public inputs and prepared verifying key");

    let verifier_prepared = build_verifier(super::prove::ProofPackage{
        proof,
        public_inputs,
        prepared_verifying_key
    });
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Load or create a keypair for the payer
    let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();
    let program_id = Pubkey::from_str("EjmMQEjv222Mz7u8jUQPC5aJ1pGDEh7xTFTupkELYV3v").unwrap(); // Replace with your actual program ID
    let instruction_data = to_vec(&ProgramInstruction::VerifyProof(verifier_prepared)).unwrap();
    let instruction = Instruction::new_with_bytes(
        program_id,
        instruction_data.as_slice(),
        vec![AccountMeta::new(payer.pubkey(), true)],
    );
    // Create and send the transaction
    let recent_blockhash = client.get_latest_blockhash().await.unwrap();
        
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );
    // Send and confirm transaction
    match client
        .send_and_confirm_transaction_with_spinner(&transaction)
        .await
    {
        Ok(signature) => {
            println!("Transaction succeeded! Signature: {}", signature);
            tx.send(()).expect("Failed to send signal");
        },
        Err(err) => println!("Transaction failed: {:?}", err),
    }


}


pub async fn insert_leaf_zkp(target_leaf: [u8; 64], merkle_proof: MerkleProof) -> bool {
     
    let (tx, rx) = oneshot::channel();

    let zkp_handle = task::spawn(async move {
        zkp_verification(tx, target_leaf, merkle_proof).await;
        true // Return true after verification
    });
    let listener_handle = task::spawn(async move {
        let result = event_listener(rx).await;
        println!("{result}");
        result.as_str() == "true"
    });

    // Wait for both tasks to complete
    let (zkp_result, listener_result) = tokio::join!(zkp_handle, listener_handle);
    zkp_result.unwrap() && listener_result.unwrap()
}