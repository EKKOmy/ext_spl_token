use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ExtMint {
    /// number of greetings
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub icon: String,
}
