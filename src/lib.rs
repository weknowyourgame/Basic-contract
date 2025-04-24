use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
        account_info::{next_account_info, AcoountInfo},
        entrypoint::ProgmramResult,
        msg,
        pubkey::Pubkey,
        entrypoint
};

pub fn counter_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let acnt = next_account_info(&mut accounts.iter()).unwrap();
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
