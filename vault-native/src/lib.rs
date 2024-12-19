use solana_program::{declare_id, entrypoint};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
    msg,
    program_error::ProgramError::InsufficientFunds,
};

declare_id!("8N7HDctX8F5qcXq849KhwPfUD2CNwiEf7ANLwfB2Sk9A");

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

#[cfg(test)]
mod tests;

entrypoint!(process_instruction);

pub fn process_instruction(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [signer, vault] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(signer.is_signer);

    let lamports: u64 = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);
    let bump = data[8];
    
    // Calculate the expected PDA
    let (expected_pda, _) = Pubkey::find_program_address(
        &[
            signer.key.as_ref(),
            &[bump],
            program_id.as_ref(),
            PDA_MARKER,
        ],
        program_id,
    );

    // Verify the vault account is the expected PDA
    assert_eq!(expected_pda, *vault.key);

    msg!(
        "Before transfer - Vault balance: {}, Signer balance: {}, Transfer amount: {}", 
        vault.lamports(), 
        signer.lamports(),
        lamports
    );

    // Check if vault has enough lamports
    if vault.lamports() < lamports {
        return Err(InsufficientFunds);
    }

    // Safe way to transfer lamports without simultaneous mutable borrows
    **vault.try_borrow_mut_lamports()? = vault.lamports().checked_sub(lamports)
        .ok_or(InsufficientFunds)?;
    **signer.try_borrow_mut_lamports()? = signer.lamports().checked_add(lamports)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    msg!(
        "After transfer - Vault balance: {}, Signer balance: {}", 
        vault.lamports(), 
        signer.lamports()
    );

    Ok(())
}