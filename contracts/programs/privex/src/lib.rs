use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111"); // placeholder; replace after deploy

#[program]
pub mod privex {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
