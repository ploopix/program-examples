use solana_program::pubkey;
use steel::Pubkey;

pub const MINIMUM_LIQUIDITY: u64 = 100;

pub const AUTHORITY_SEED: &[u8] = b"authority";

pub const LIQUIDITY_SEED: &[u8] = b"liquidity";

pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
