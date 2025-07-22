use anchor_lang::prelude::*;

declare_id!("FZqjVHLRcdVJtU3uNcQSDt52Ao1uR9eTf7aDkY9oyJV3");

#[program]
pub mod hello_world {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let hello_world_account = &mut ctx.accounts.hello_world_account;
        hello_world_account.greeting = "Hello World".to_string();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// TIP: space = account discriminator + HelloWorldAccount::INIT_SPACE
    /// Use InitSpace macro to calculate the space instead of doing it manually
    #[account(
        init,
        payer = signer,
        // The `init` constraint with `InitSpace` derive macro automatically handles the space calculation.
        // You don't need to manually specify `space = 8 + HelloWorldAccount::INIT_SPACE`
        // when using the `#[derive(InitSpace)]` macro on the account struct.
        // Anchor will automatically calculate the space needed including the discriminator.
        space = 8 + 32 + 4, // 8 for discriminator, 32 for max_len string, 4 for string length prefix
    )]
    pub hello_world_account: Account<'info, HelloWorldAccount>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct HelloWorldAccount {
    #[max_len(32)]
    pub greeting: String,
}
