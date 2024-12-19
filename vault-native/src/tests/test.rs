use solana_sdk::{
    account::ReadableAccount,
    instruction::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
};

use crate::tests::setup;

#[test]
fn withdraw_test() {
    // Setup program and accounts
    let (program_id, mollusk) = setup();
    let signer = Pubkey::new_from_array([0x2; 32]);
    
    // Find the correct bump value by searching
    let (vault, bump) = Pubkey::find_program_address(
        &[
            signer.as_ref(),
            &[255], // Initial bump guess
            program_id.as_ref(),
            b"ProgramDerivedAddress",
        ],
        &program_id,
    );

    // Create signer account with initial lamports
    let initial_signer_balance = mollusk
        .sysvars
        .rent
        .minimum_balance(spl_token::state::Account::LEN);
    let signer_account = crate::tests::create_account(
        initial_signer_balance,
        spl_token::state::Account::LEN,
        &program_id,
    );

    // Create vault account with initial lamports
    let initial_vault_balance = 300_000u64;
    let vault_account = crate::tests::create_account(
        initial_vault_balance,
        spl_token::state::Account::LEN,
        &program_id,
    );

    // Prepare withdraw data with correct bump
    let withdraw_amount = 300_000u64;
    let mut data = withdraw_amount.to_le_bytes().to_vec();
    data.push(bump);

    // Prepare withdraw instruction
    let withdraw_instruction = Instruction::new_with_bytes(
        program_id,
        &data,
        vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(vault, false),
        ],
    );
    
    let result = mollusk.process_instruction_chain(
        &[withdraw_instruction],
        &vec![(signer, signer_account.clone()), (vault, vault_account.clone())],
    );

    assert!(
        !result.program_result.is_err(),
        "Withdraw instruction failed: {:?}",
        result.program_result
    );
    println!("Compute Units: {}", result.compute_units_consumed);

}