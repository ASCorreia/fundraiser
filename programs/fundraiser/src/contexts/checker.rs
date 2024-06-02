use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{
        transfer, 
        Mint, 
        Token, 
        TokenAccount, 
        Transfer
    }
};

use crate::state::Fundraiser;

#[derive(Accounts)]
pub struct CheckContributions<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    pub mint_to_raise: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [b"fundraiser".as_ref(), maker.key().as_ref()],
        bump = fundraiser.bump,
    )]
    pub fundraiser: Account<'info, Fundraiser>,
    #[account(
        mut,
        associated_token::mint = mint_to_raise,
        associated_token::authority = fundraiser,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_to_raise,
        associated_token::authority = maker,
    )]
    pub maker_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CheckContributions<'info> {
    pub fn check_contributions(&self) -> Result<()> {
        if self.vault.amount >= self.fundraiser.amount_to_raise {

            let cpi_program = self.token_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.maker_ata.to_account_info(),
                authority: self.fundraiser.to_account_info(),
            };

            let signer_seeds: [&[&[u8]]; 1] = [&[
                b"fundraiser".as_ref(),
                self.maker.to_account_info().key.as_ref(),
                &[self.fundraiser.bump],
            ]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, &signer_seeds);

            transfer(cpi_ctx, self.vault.amount)?;

            self.fundraiser.close(self.maker.to_account_info())?;
        }

        Ok(())
    }
}