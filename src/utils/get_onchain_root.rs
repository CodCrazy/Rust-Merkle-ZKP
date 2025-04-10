use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, instruction::{AccountMeta, Instruction}, pubkey::Pubkey, signature::{Keypair, Signer}, signer::EncodableKey, system_program, transaction::Transaction
};
use solana_sdk::signature::Signature;
use std::str::FromStr;
use borsh::{BorshDeserialize, BorshSerialize, to_vec};

pub fn get_current_root() -> Result<Vec<u8>, String> {
    // Program ID (replace with your actual program ID)
    let program_id = Pubkey::from_str("9guwSzLJSkomxdbTM6TfKTF3KYSDxLNeSsCRdPaBGVpU").unwrap();
     
    // Connect to the Solana devnet
    let rpc_url = String::from("https://api.devnet.solana.com");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
    
    // Generate a new keypair for the payer
    let payer = Keypair::read_from_file("src/wallet-keypair.json").unwrap();
    let seed_text = "root_hashes";
    // Derive PDA
    let (pda, _) = Pubkey::find_program_address(
        &[payer.pubkey().as_ref(), seed_text.as_bytes().as_ref()],
        &program_id,
    );
    match client.get_account(&pda) {
        Ok(account) => Ok(account.data),
        Err(_) => Err("PDA account does not exist!".to_string()), // Return an error message here
    }
}
