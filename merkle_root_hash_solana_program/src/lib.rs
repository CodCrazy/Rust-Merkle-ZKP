use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};
entrypoint!(process_instruction);
#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramInstruction {
    MerkleRootHash([u8; 32], [u8; 32], [u8; 32]),
    CreateAccount(bool)
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct HashAccount {
    ddid_root: [u8; 32],
    merchant_root: [u8; 32],
    merchant_record_root: [u8; 32]
}


pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::try_from_slice(instruction_data).unwrap();
    match instruction {
        ProgramInstruction::MerkleRootHash(ddid_root, merchant_root, merchant_record_root) => {
            return update_hash_account(program_id, accounts, ddid_root, merchant_root, merchant_record_root);
        }
        ProgramInstruction::CreateAccount(_is_create) => {
            return create_root_hash_account(program_id, accounts, instruction_data);
        }
    }
}

fn create_root_hash_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let signer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let size = std::mem::size_of::<HashAccount>() as u64;
    let lamports = (Rent::get()?).minimum_balance(size.try_into().unwrap());
    let seed_text = "root_hashes";
    // Derive PDA
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[signer.key.as_ref(), seed_text.as_bytes().as_ref()],
        program_id,
    );
    assert!(new_account.key == &pda, "new_account info is incorrect!");
    invoke_signed(
        &create_account(
            signer.key,
            new_account.key,
            lamports,
            size,
            program_id,
        ),
        &[
            signer.clone(), 
            new_account.clone(), 
            system_program.clone()
        ],
        &[&[
            signer.key.as_ref(),
            seed_text.as_bytes().as_ref(),
            &[bump_seed]
        ]],
    )?;
    msg!("PDA account created!: {:?}", pda);
    Ok(())
}

fn update_hash_account(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    ddid_root: [u8; 32],
    merchant_root: [u8; 32],
    merchant_record_root: [u8; 32],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let _signer = next_account_info(accounts_iter)?;
    let new_account = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let mut account_data =
        HashAccount::try_from_slice(&new_account.data.borrow()).unwrap_or(HashAccount {
            ddid_root: [0u8; 32],
            merchant_root: [0u8; 32],
            merchant_record_root: [0u8; 32],
        });
    account_data.ddid_root = ddid_root;
    account_data.merchant_root = merchant_root;

    account_data.merchant_record_root = merchant_record_root;

    msg!("Changed data to: root_hashes: ddid_root: {:?}, merchant_root: {:?}, merchant_record_root: {:?}", ddid_root, merchant_root, merchant_record_root);

    msg!("Serializing account");
    account_data.serialize(&mut &mut new_account.data.borrow_mut()[..])?;
    msg!("State account serialized");
    Ok(())
}


