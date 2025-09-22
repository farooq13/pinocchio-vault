use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address, ProgramResult};
use pinocchio_system::instructions::Transfer;



pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
}