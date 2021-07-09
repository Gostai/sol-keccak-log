 //! Program instruction processor
//extern crate keccak_rust;
use keccak_rust::*;
use std::str;


use solana_program::{
    account_info::{ AccountInfo},
    entrypoint::ProgramResult,
    log::{sol_log_compute_units},
    msg,   
    pubkey::Pubkey,
};


/// Instruction processor
pub fn process_instruction(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    msg!("Calculating keccak for instruction_data:");
    
    msg!("Sourse string is : {}", str::from_utf8(instruction_data).unwrap());
    
    // Log a public key
    program_id.log();

    // Log the number of compute units remaining that the program can consume.
    sol_log_compute_units();
    
    //Calculate keccak
    let mut keccak = Keccak::new(SecurityLevel::SHA256, StateBitsWidth::F1600);
    keccak.append(& instruction_data);
    let b64 = base64::encode(keccak.hash());
    
    msg!("Base 64 encoding is:{} ", b64);
    
    // Log the number of compute units remaining that the program can consume.
    sol_log_compute_units();
    
    msg!("Hash for 'Hello world' really is: 7WwRsLW4CJYN8m9b/EcdBMGZWw/9IFWSWtG+KNa6rf0=");
          

    Ok(())
}
