use anchor_lang::prelude::*;

declare_id!("FMgYedYZQssbFycVvg4W17hbaeoT8JCa3DRmAYNXyCr3");

#[program]
pub mod temp_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
