use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{self, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked},
};
use spl_token_2022::instruction::AuthorityType; 
use std::mem::size_of;

declare_id!("JArVcBUHioy1shUQVUQJGGp83sR39zzFLsYkHpQLUMn8");

#[program]
pub mod owned_nft {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let nft_info = &mut ctx.accounts.nft_info;
        nft_info.count = 0; 
        nft_info.owner = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<()> {

        require!(
            ctx.accounts.signer.key() == ctx.accounts.nft_info.owner,
            NFTError::Unauthorized
        );

        let signer_seeds: &[&[&[u8]]] = &[&[&ctx.accounts.nft_info.count.to_le_bytes(), &[ctx.bumps.mint]]];

        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

        token_interface::mint_to(cpi_context, 1)?;

        let counter = &mut ctx.accounts.nft_info;
        counter.count += 1;

        let nft_data = &mut ctx.accounts.nft_data;
        nft_data.name = name;
        nft_data.symbol = symbol;
        nft_data.uri = uri;
        nft_data.minter = ctx.accounts.signer.key();
        nft_data.current_holder = ctx.accounts.receiver.key();


        // set authority to None to make sure nobody can mint again, so the supply will be only 1
        let cpi_accounts_set_auth = token_interface::SetAuthority {
            account_or_mint: ctx.accounts.mint.to_account_info(),
            current_authority: ctx.accounts.mint.to_account_info(), 
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts_set_auth).with_signer(signer_seeds);
        token_interface::set_authority(
            cpi_ctx,
            AuthorityType::MintTokens,
            None 
        )?;

        Ok(())
    }


    pub fn transfer_nft(ctx: Context<TransferNFT>) -> Result<()> {

        let nft_data = &mut ctx.accounts.nft_data;
        nft_data.current_holder = ctx.accounts.to.key();
        let decimals = ctx.accounts.mint.decimals;

        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from_token_account.to_account_info(),
            to: ctx.accounts.to_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token_interface::transfer_checked(cpi_ctx, 1, decimals)?;

        Ok(())
    }
}


#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(
        init,
        payer = signer,
        space = size_of::<NFTInfo>() + 8, 
        seeds = [b"nft_info".as_ref()],
        bump
    )]
    pub nft_info: Account<'info, NFTInfo>,

    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {

    #[account(
        init,
        payer = signer,
        space = size_of::<NFTData>() + 8,
        seeds = [mint.key().as_ref()], 
        bump
    )]
    pub nft_data: Account<'info, NFTData>,

    #[account(
        mut, 
        seeds = [b"nft_info".as_ref()],
        bump)]
    pub nft_info: Account<'info, NFTInfo>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = mint,
        seeds = [&nft_info.count.to_le_bytes()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>, 
    
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = receiver,
        associated_token::token_program = token_program,
    )]
    pub receiver_token_account: InterfaceAccount<'info, TokenAccount>, 
    
    /// CHECK: This is the receiver's public key, used as the authority for the ATA, not deserialized
    pub receiver: AccountInfo<'info>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>, 
    
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferNFT<'info> {

    #[account(
        mut,
        seeds = [mint.key().as_ref()], 
        bump
    )]
    pub nft_data: Account<'info, NFTData>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub from_token_account: InterfaceAccount<'info, TokenAccount>, 

    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = to,
        associated_token::token_program = token_program,
    )]
    pub to_token_account: InterfaceAccount<'info, TokenAccount>, 
    
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>, 
    
    #[account(mut)]
    pub signer: Signer<'info>,

    /// CHECK: This is the recipient's public key, used as the authority for the ATA, not deserialized
    pub to: AccountInfo<'info>,

    pub token_program: Interface<'info, TokenInterface>, 
    
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct NFTData {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub minter: Pubkey,
    pub current_holder: Pubkey,
}


#[account]
#[derive(Default)]
pub struct NFTInfo {
    pub count: u64, 
    pub owner: Pubkey,
}


#[error_code]
pub enum NFTError {
    #[msg("Unauthorized: Only owner can perform this action")]
    Unauthorized,
    #[msg("Invalid name length")]
    InvalidNameLength,
    #[msg("Invalid symbol length")]
    InvalidSymbolLength,
    #[msg("Invalid URI length")]
    InvalidURILength,
}