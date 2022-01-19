use anchor_lang::prelude::*;
use switchboard_aggregator::decimal::SwitchboardDecimal;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod oracle_solana_example {
    use super::*;

    pub fn update_btc_price(ctx: Context<UpdateBTCPrice>) -> ProgramResult {
        Ok(())
    }

    pub fn init_btc_price(ctx: Context<InitBTCPrice>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitBTCPrice<'info> {
    #[account(zero)]
    pub btc_price_account: AccountLoader<'info, BTCPriceAccountData>
}

#[derive(Accounts)]
pub struct UpdateBTCPrice {
}

#[account(zero_copy)]
pub struct BTCPriceAccountData {
    pub val: SwitchboardDecimal,
}
