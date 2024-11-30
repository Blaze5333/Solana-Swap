pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("D7JfecGidcYYq73haTanxWbbXroXdRhbf9USuz4vNQKM");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>,token_a_offered_amount:u64,token_b_wanted_amount:u64,id:u64) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(&ctx,token_a_offered_amount)?;
        instructions::make_offer::save_offer(ctx,token_b_wanted_amount,id)?;
        Ok(())
    }
    pub fn take_offer(ctx:Context<TakeOffer>)->Result<()>{
        take_offer::send_wanted_tokens_to_maker(&ctx)?;
        take_offer::withdraw_and_close_vault(&ctx)?;
        Ok(())
    }
}
