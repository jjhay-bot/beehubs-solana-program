use anchor_lang::prelude::*;

declare_id!("7Q4EY4QVj1o7MyNdzsPed1XYycvgaWmBpnishC8m4yQK");

#[program]
pub mod beehubs {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }

    // The function now accepts a gif_link param from the user. We also reference the user from the Context
    pub fn add_gif(ctx: Context<AddGif>, gif_link: String, gif_tag: String, gif_name: Option<String>, timestamp: Option<String>, author: Option<String>, author_avatar: Option<String>, style: Option<String>) -> Result <()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            gif_tag: gif_tag.to_string(),
            user_address: *user.to_account_info().key,
            gif_name,
            timestamp,
            author,
            author_avatar,
            style
        };
        
        // Add it to the gif_list vector.
        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }
}


// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub gif_tag: String, // New field added
    pub gif_name: Option<String>,
    pub timestamp: Option<String>,
    pub author: Option<String>,
    pub author_avatar: Option<String>,
    pub style: Option<String>
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    // Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}


// #[blah] =>  macros , basically attach code to our module. It's sorta like "inheriting" a class.
// pub mod => module , way to define a collection of functions and variables
