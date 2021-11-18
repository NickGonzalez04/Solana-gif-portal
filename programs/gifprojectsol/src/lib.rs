use anchor_lang::prelude::*;
use anchor_lang::AccountsClose;

// our program_id to run the program in Solana
declare_id!("H1XSeK4joK6pEF4sUd8oyBo53rhHpE7ARFpWN3uAr144");


#[program]
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

    pub fn delete_list(ctx: Context<InitList>) -> ProgramResult {
        let user = &ctx.accounts.user;
        let base_account = &mut ctx.accounts.base_account;
        base_account.close(user.to_account_info())?;
        // let remove = base_account.gif_list
        Ok(())
    }

    // New fn instance that adds new gif
    pub fn add_new_gif(ctx: Context<AddNewGif>, gif_link: String ) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.gif_list.push(GifItem {
            gif_link,
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
        gif.upvotes += 1;
        Ok(())
    }
}



#[error]
pub enum Err {
    #[msg("No Gif with that url")]
    NoGifFound,
}

// instru macro informing Anchor to deserialzie the instruction args so accounts can use the values
#[derive(Accounts)]
#[instruction(bump: u8)]
    pub struct InitList<'info> {
        #[account(init, payer=user, seeds=[
            b"ngGif2",
            user.to_account_info().key.as_ref()],
            bump=bump,
            space = 9000)]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
}

// Delete a gif Item
#[derive(Accounts)]
pub struct DeleteList<'info> {
    #[account(mut, seeds=[
        b"ngGif2",
        user.to_account_info().key.as_ref()], 
        bump=base_account.bump
    )]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
}


// Data that is in the Add new gif func.
 #[derive(Accounts)]
    pub struct AddNewGif<'info> {
        #[account(mut, seeds=[
        b"ngGif2",
        list_owner.to_account_info().key.as_ref()  
        ], bump=base_account.bump)]
        pub base_account: Account<'info, BaseAccount>,
        pub list_owner: AccountInfo<'info>,
        pub user: Signer<'info>,
    }


// Gif is the format behind saved gifs
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GifItem {
    gif_link: String,
    authority: Pubkey,
    upvotes: u64,
}

// Instructing Solana what we want to store to an account.
#[account]
    pub struct BaseAccount {
        pub bump: u8,
        pub gif_list: Vec<GifItem>,
}
