use anchor_lang::prelude::*;
use switchboard_aggregator::decimal::SwitchboardDecimal;
use solana_program;
//use bytemuck::Zeroable;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod oracle_solana_example {
    use super::*;

    pub fn update_btc_price(ctx: Context<UpdateBTCPrice>) -> ProgramResult {
        Ok(())
    }

    pub fn init_btc_price(ctx: Context<InitBTCPrice>) -> ProgramResult {
        let mut btc_price_account = &mut ctx.accounts.btc_price_account.load_init()?;
        btc_price_account.val = SwitchboardDecimal::from_f64(0f64);
        Ok(())
    }
}

const BTC_PRICE_SEED: &[u8] = b"BTCPrice";

#[derive(Accounts)]
#[instruction(params: InitBTCPriceParams)] // rpc parameters hint
pub struct InitBTCPrice<'info> {
    #[account(
        init,
        seeds = [
            BTC_PRICE_SEED,
        ],
        bump = params.btc_price_bump,
        payer = payer)]
    pub btc_price_account: AccountLoader<'info, BTCPriceAccountData>,
    #[account(signer)]
    pub payer: AccountInfo<'info>,
    #[account(address = solana_program::system_program::ID)]
    pub system_program: AccountInfo<'info>,
}

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitBTCPriceParams {
    pub btc_price_bump: u8, 
}

#[derive(Default, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateBTCPriceParams {
}

#[derive(Accounts)]
#[instruction(params: UpdateBTCPriceParams)]
pub struct UpdateBTCPrice<'info> {
    pub pyth_btc_feed: AccountInfo<'info>,
    pub switchboard_btc_feed: AccountInfo<'info>
}

#[account(zero_copy)]
#[derive(Default)]
pub struct BTCPriceAccountData {
    pub val: SwitchboardDecimal,
}
