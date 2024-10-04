use anchor_lang::prelude::*;
use anchor_spl::token::{mint_to, MintTo, Mint, TokenAccount, Token};
use anchor_spl::associated_token::AssociatedToken;

// constants here
const ANCHOR_DISCRIMINATOR: usize = 8;
const PUBKEY_SIZE: usize = 32;
const U8_SIZE: usize = 1;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_DESCRIPTION_LENGTH: usize = 50;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_RATING: u8 = 5;
const MIN_RATING: u8 = 1;

declare_id!("4fKehYqNQMRoLfGLNRQjwJWs3sZA1xTGM7QSD7mLaZmM");

#[program]
pub mod anchor_token_movie_review {
    use super::*;

    pub fn add_movie_review(
        ctx: Context<AddMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        // We require that the rating is between 1 and 5
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);
        // we require that the title is not longer than 20 characters
        require!(title.len() <= MAX_TITLE_LENGTH, MovieReviewError::TitleTooLong);
        // we require that the description is not longer than 50 characters
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);

        // log the process and the fields
        msg!("Movie Review Account created successfully");
        msg!("Title is: {}", title);
        msg!("Description is : {}", description);
        msg!("Rating is: {}", rating);

        // instantiate a default
        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.description = description;
        movie_review.rating = rating;

        // Starting mint process when users add a movie review
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    authority: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                },
                &[&[
                    "mint".as_bytes(),
                    &[ctx.bumps.mint]
                ]]
            ),
            10*10^6
        )?;

        // log the completion of mint
        msg!("Minted tokens");
        Ok(())
    }


    // The update movie review instruction
    pub fn update_movie_review(
        ctx: Context<UpdateMovieReview>,
        title: String,
        description: String,
        rating: u8,
    ) -> Result<()> {
        // we require that the rating is within the allowable rating validity range
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);
        // We require that the description length is not longer than 50 characters
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);

        // log the updating process and changes being made
        msg!("Movie Review Account space reallocated");
        msg!("Title is: {}", title);
        msg!("Description is: {}", description);
        msg!("Rating is: {}", rating);

        // instantiates the update
        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.description = description;
        movie_review.rating = rating;

        Ok(())
    }


    // The delete movie review instruction
    pub fn delete_movie_review(
        _ctx: Context<DeleteMovieReview>,
        title: String,
    ) -> Result<()> {
        // log the  process ongoing
        msg!("Movie review for {} is being Deleted", title);
        Ok(())
    }


    // the Initialize minting instruction
    pub fn initialize_token_mint(_ctx: Context<InitializeMint>) -> Result<()> {
        // log the process
        msg!("Token mint is being initialized");
        Ok(())
    }
}


#[derive(Accounts)]
#[instruction(title: String, description: String)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = MovieAccountState::INIT_SPACE + title.len() + description.len(),
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    #[account(
        seeds = ["mint".as_bytes()],
        bump,
        mut,
    )]
    pub mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer,
    )]
    pub token_account: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        close = initializer,
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


// The mint account
#[derive(Accounts)]
pub struct InitializeMint<'info> {
    #[account(
        init,
        seeds = ["mint".as_bytes()],
        bump,
        payer = user,
        mint::decimals = 6,
        mint::authority = mint,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}



// We implement the movie Account here
#[account]
pub struct MovieAccountState {
    pub reviewer: Pubkey,
    pub rating: u8,
    pub title: String,
    pub description: String,
}

impl Space for MovieAccountState {
    const INIT_SPACE: usize = ANCHOR_DISCRIMINATOR + PUBKEY_SIZE + U8_SIZE + STRING_LENGTH_PREFIX + STRING_LENGTH_PREFIX;
}

#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong,
}


#[derive(Accounts)]
pub struct Initialize {}
