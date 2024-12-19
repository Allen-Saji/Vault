use five8_const::decode_32_const;
use pinocchio::entrypoint;
use pinocchio::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
    msg,
};
use solana_nostd_sha256::hashv;

#[cfg(test)]
mod tests;

const ID: [u8; 32] = decode_32_const("8N7HDctX8F5qcXq849KhwPfUD2CNwiEf7ANLwfB2Sk9A");

const PDA_MARKER: &[u8; 21] = b"ProgramDerivedAddress";

entrypoint!(withdraw);

/// Handles withdrawing funds from a PDA that has previously had lamports deposited to it.
pub fn withdraw(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let [signer, vault] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    assert!(signer.is_signer());

    let lamports: u64 = unsafe { *(data.as_ptr() as *const u64) };
    let bump = data[8];
    let pda = hashv(&[
        signer.key().as_ref(),
        &[bump],
        ID.as_ref(),
        PDA_MARKER,
    ]);
    
    unsafe{
        msg!(
        "Before transfer - Vault balance: {:?}, Signer balance: {:?}, Transfer amount: {}", 
        vault.borrow_lamports_unchecked(),
        signer.borrow_lamports_unchecked(),
        lamports
        );
    }

    assert_eq!(&pda, vault.key().as_ref());

    unsafe {
        *vault.borrow_mut_lamports_unchecked() -= lamports;
        *signer.borrow_mut_lamports_unchecked() += lamports;
    }

    unsafe {
        msg!(
        "After transfer - Vault balance: {}, Signer balance: {}", 
        vault.borrow_lamports_unchecked(), 
        signer.borrow_lamports_unchecked()
        );
    }

    Ok(())
}