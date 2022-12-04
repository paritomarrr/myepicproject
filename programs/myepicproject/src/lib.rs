use anchor_lang::prelude::*;

declare_id!("2z1kUrrP21ZBUKDBXz9t4JXan17EiJ2LEY3AvgjR1GVt");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Get a reference to the account
        let base_account = &mut ctx.accounts.base_account;
        // initialise total gifs
        base_account.total_gifs = 0;
        Ok(())
    }
}
// attach certain variables to the initialise context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_rogram: Program <'info, System>,
}

// tell solana what we want to store on this account
#[account]
pub struct BaseAccount {
    // total gif can hold integer var
    pub total_gifs: u64,

}
