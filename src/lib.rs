use errors::SolNftError;
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

mod errors;

// declare and export the program's entrypoint
entrypoint!(process_instruction);

// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // log a message to the blockchain
    msg!("Hello from sol_nft!");

    // Initialize an NFT
    let nft = Nft{
        name: "MyNFT".to_string(),
        symbol: "MNFT".to_string(),
        uri: "ipfs:://my-nft.example".to_string(),
        capacity: 10,
    }

    // Send the NFT to the second account provided in accounts, i.e. accounts[1]
    assert!(accounts.len == 2)
    let to = accounts[1]


    // gracefully exit the program
    Ok(())
}

/// Transfers lamports from one account (must be program owned)
/// to another account. The recipient can by any account
fn transfer_service_fee_lamports(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount_of_lamports: u64,
) -> ProgramResult {
    // Does the from account have enough lamports to transfer?
    if **from_account.try_borrow_lamports()? < amount_of_lamports {
        return Err(SolNftError::InsufficientFundsForTransaction.into());
    }
    // Debit from_account and credit to_account
    **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    Ok(())
}

/// Primary function handler associated with instruction sent
/// to your program
fn instruction_handler(accounts: &[AccountInfo]) -> ProgramResult {
    // Get the 'from' and 'to' accounts
    let account_info_iter = &mut accounts.iter();
    let from_account = next_account_info(account_info_iter)?;
    let to_service_account = next_account_info(account_info_iter)?;

    // Extract a service 'fee' of 5 lamports for performing this instruction
    transfer_service_fee_lamports(from_account, to_service_account, 5u64)?;

    // Perform the primary instruction
    // ... etc.

    Ok(())
}


/// A basic NFT implementation
struct Nft {
    /// The name of the NFT
    name: String,
    /// The shorthand symbol of the NFT
    symbol: String,
    /// The uri of the image backing the NFT
    uri: String,
    /// The max amount of NFTs that can be minted
    capacity: u64,
}