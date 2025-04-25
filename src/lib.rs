use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Counter {
    pub count: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum CounterInstruction {
    Increament(u32),
    Decreament(u32),
}

pub fn counter_program(
    program_id: &Pubkey,                          // address of the program users are supposed to specify this when they interact with this contract
    accounts: &[AccountInfo],                    // an array of input addys the user uses to interact with they call this func 
    instruction_data: &[u8],                    // the actual thing the user wants to do --> contains info like what to do (like add to a counter, remove to a counter)
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let account = next_account_info(account_info_iter)?;

    let mut counter = Counter::try_from_slice(&account.data.borrow())?;

    match CounterInstruction::try_from_slice(instruction_data)? {
        CounterInstruction::Increament(amount) => {
            counter.count += amount;
        }
        CounterInstruction::Decreament(amount) => {
            counter.count -= amount;
        }
    }

    counter.serialize(&mut &mut account.data.borrow_mut())?;
    Ok(())
}

