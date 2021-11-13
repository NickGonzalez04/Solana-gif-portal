// Simiular to import
use anchor_lang::prelude::*;

// our program_id to run the program in Solana
declare_id!("AV17oYk1DpdFkhioddypZhLWzxuTtSGAcJ2RbrETqFxP");

#[program]
// defines our collection of functions
pub mod gifprojectsol {
    use super::*;
    pub fn init_list(ctx: Context<InitList>, bump: u8) -> ProgramResult {
        // Getting reference to the account
        // `mut` allows for an immutable reference to the base account (in order to make changes to it)
        let base_account = &mut ctx.accounts.base_account;
        base_account.bump = bump;
        base_account.gif_list = Vec::new();
        Ok(())
    }

    // New fn instance that adds new gif
    pub fn add_new_gif(ctx: Context<AddNewGif>, gif_link: String ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.gif_list.push(GifItem {
            gif_link: gif_link.to_string(),
            authority: *ctx.accounts.user.to_account_info().key,
            upvotes: 0,
        });

        Ok(())
    }

    // Upvote Gif fn
    pub fn up_vote_gif(ctx: Context<AddNewGif>, gif_link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let gif = base_account
            .gif_list
            .iter_mut()
            .find(|gif| gif.gif_link == gif_link)
            .ok_or(Err::NoGifFound)?;
            gif.upvotes +=1;
            Ok(())
    }
}



#[error]
pub enum Err {
    #[msg("No Gif with that url")]
    NoGifFound,
}
#[derive(Accounts)]
// instru macro informing ANchor to deserialzie the instruction args so accounts can use the values
#[instruction(bump: u8)]
    pub struct InitList<'info> {
        #[account(init, payer = user, seeds=[
            b"ngGifme",
            user.to_account_info().key.as_ref()],
            bump=bump,
            space = 9000)]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program <'info, System>,
}


// Data that is in the Add new gif func.
 #[derive(Accounts)]
    pub struct AddNewGif<'info> {
        #[account(mut, seeds=[
        b"ngGifme",
        user.to_account_info().key.as_ref()  
        ], bump=base_account.bump)]
        pub base_account: Account<'info, BaseAccount>,
        pub user: Signer<'info>,
    }


// Gif is the format behind saved gifs
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GifItem {
    pub gif_link: String,
    pub authority: Pubkey,
    pub upvotes: u64,
}

// Instructing Solana what we want to store to an account.
#[account]
    pub struct BaseAccount {
        pub total_gifs: u64,
        pub bump: u8,
        // Vector (array) holding our gifs
        pub gif_list: Vec<GifItem>,
}
