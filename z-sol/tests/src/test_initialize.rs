use anchor_lang::prelude::*;
use std::str::FromStr;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig,
        pubkey::Pubkey,
        signature::{read_keypair_file, Signer},
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

    // Create client
    let client = Client::new_with_options(
        Cluster::Localnet,
        &payer,
        CommitmentConfig::processed()
    );
    let program = client.program(Pubkey::from_str(program_id).unwrap())
        .expect("Failed to create program client");

    // Test recording a game result
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Generate PDA for game result account
    let payer_pubkey = payer.pubkey();
    let seeds = [
        b"game_result",
        payer_pubkey.as_ref(),
        &timestamp.to_le_bytes(),
    ];
    let (game_result_pda, _bump) = 
        Pubkey::find_program_address(&seeds, &program.id());

    // Record a game result
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
    
    println!("Game result recorded! Tx: {}", record_tx);

    // Verify the recorded data
    let game_result_account = program
        .account::<z_sol::GameResult>(game_result_pda)
        .expect("Failed to fetch game result account");

    assert_eq!(game_result_account.player, payer.pubkey());
    assert_eq!(game_result_account.result, GameOutcome::Win);
    assert_eq!(game_result_account.timestamp, timestamp);
    
    println!("Game result verified successfully!");
}