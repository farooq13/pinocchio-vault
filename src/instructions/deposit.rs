use pinocchio::{account_info::AccountInfo, instruction::{Account, Seed, Signer}, program_error::ProgramError, pubkey::Pubkey, ProgramResult};
use pinocchio_system::instructions::Transfer;


pub struct WithdrawAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
    pub bump: [u8; 1]
}

