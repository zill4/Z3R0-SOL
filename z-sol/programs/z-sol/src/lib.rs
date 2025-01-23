use anchor_lang::prelude::*;

declare_id!("3kVGkkFSJZEpPhiDuGBXbYRC3jLz66EffCJeEWy7NPbQ");

#[program]
pub mod z_sol {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Starting Game Results Program Initialization");
        msg!("Game Results Program Initialized Successfully!");
        Ok(())
    }

    pub fn record_game_result(
        ctx: Context<RecordGameResult>,
        result: GameOutcome,
        timestamp: i64,
    ) -> Result<()> {
        let game_result = &mut ctx.accounts.game_result;
        
        msg!("Recording new game result...");
        game_result.player = ctx.accounts.player.key();
        game_result.result = result.clone();
        game_result.timestamp = timestamp;
        
        msg!("Game result recorded for player: {:?}", game_result.player);
        msg!("Result: {:?}", result);
        msg!("Timestamp: {}", timestamp);
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
#[instruction(result: GameOutcome, timestamp: i64)]
pub struct RecordGameResult<'info> {
    #[account(
        init,
        payer = player,
        space = 8 + GameResult::SPACE,
        seeds = [b"game_result", player.key().as_ref(), &timestamp.to_le_bytes()],
        bump
    )]
    pub game_result: Account<'info, GameResult>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct GameResult {
    pub player: Pubkey,            // 32 bytes
    pub result: GameOutcome,       // 1 byte
    pub timestamp: i64,            // 8 bytes
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug)]
pub enum GameOutcome {
    Win,
    Loss,
    Draw,
}

// Calculate space required for GameResult
impl GameResult {
    pub const SPACE: usize = 32 + 1 + 8; // player + result + timestamp
}