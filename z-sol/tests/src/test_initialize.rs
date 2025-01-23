use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{Keypair, read_keypair_file},
        system_program,
    },
    Client, Cluster,
};
use z_sol::GameOutcome;

#[test]
fn test_game_program() {
    println!("\n=== Starting Game Results Program Tests ===\n");
    
    // Setup
    let program_id = "3kVGkkFSJZEpPhiDuGBXbYRC3jLz66EffCJeEWy7NPbQ";
    println!("Program ID: {}", program_id);
    
    println!("Loading wallet...");
    let anchor_wallet = std::env::var("ANCHOR_WALLET")
        .expect("ANCHOR_WALLET environment variable not found!");
    let payer = read_keypair_file(&anchor_wallet)
        .expect("Failed to read keypair file");
    println!("Wallet loaded: {}", payer.pubkey());

    println!("Creating client connection...");
    let client = Client::new_with_options(
        Cluster::Localnet,
        &payer,
        CommitmentConfig::processed()
    );
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id)
        .expect("Failed to create program client");
    println!("Client connection established");

    // Test 1: Initialize Program
    println!("\n--- Test 1: Program Initialization ---");
    let init_tx = program
        .request()
        .accounts(z_sol::accounts::Initialize {})
        .args(z_sol::instruction::Initialize {})
        .send()
        .expect("Failed to initialize program");
    println!("Program initialized successfully!");
    println!("Initialize transaction signature: {}", init_tx);

    // Test 2: Record Game Result
    println!("\n--- Test 2: Recording Game Result ---");
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    println!("Using timestamp: {}", timestamp);
    
    // Generate PDA for game result account
    let seeds = [
        b"game_result",
        payer.pubkey().as_ref(),
        &timestamp.to_le_bytes(),
    ];
    let (game_result_pda, _bump) = Pubkey::find_program_address(&seeds, &program_id);
    println!("Generated game result PDA: {}", game_result_pda);

    println!("Recording WIN result...");
    let record_tx = program
        .request()
        .accounts(z_sol::accounts::RecordGameResult {
            game_result: game_result_pda,
            player: payer.pubkey(),
            system_program: system_program::ID,
        })
        .args(z_sol::instruction::RecordGameResult {
            result: GameOutcome::Win,
            timestamp,
        })
        .send()
        .expect("Failed to record game result");
    
    println!("Game result recorded successfully!");
    println!("Record transaction signature: {}", record_tx);
    
    // Optional: Add verification of the recorded data
    println!("\n=== All Tests Completed Successfully ===\n");
}