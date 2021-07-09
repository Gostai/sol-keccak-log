use {
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{
    account::Account,
    signature::Signer, 
    transaction::Transaction,    
    },
    sol_keccak::processor::{process_instruction},
    std::str::FromStr,
};
use std::mem;

#[tokio::test]
async fn test_logging() {
    
    let program_id = Pubkey::from_str("Keccak1111111111111111111111111111111111111").unwrap();
    let hash_pubkey = Pubkey::new_unique();
    
    let mut program_test = ProgramTest::new(
        "calculating_keccak", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        hash_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
    
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &b"Hello world",
            vec![AccountMeta::new(hash_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
    
    
} 
