use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

/// Basic "Hello, Solana!" example instruction.
/// Prints a confirmation message to verify that the program runs correctly.
pub fn process_hello_world(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Hello, Solana BPF world! 🧭");
    Ok(())
}
