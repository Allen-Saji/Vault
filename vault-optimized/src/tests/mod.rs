#[cfg(test)]
mod test;


use mollusk_svm::Mollusk;
use solana_sdk::account::AccountSharedData;
use solana_sdk::pubkey::Pubkey;


pub fn setup() -> (Pubkey, Mollusk) {
    let program_id = Pubkey::new_from_array(five8_const::decode_32_const(
        "8N7HDctX8F5qcXq849KhwPfUD2CNwiEf7ANLwfB2Sk9A",
    ));
    let mut mollusk = Mollusk::new(&program_id, "target/deploy/vault");
    mollusk_token::token::add_program(&mut mollusk);
    (program_id, mollusk)
}

pub fn create_account(lamports: u64, data_len: usize, owner: &Pubkey) -> AccountSharedData {
    AccountSharedData::new(lamports, data_len, owner)
}
