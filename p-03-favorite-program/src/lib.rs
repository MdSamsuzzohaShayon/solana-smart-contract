// Tutorial - https://youtu.be/amAq-WHAFs8?t=622
// This imports all necessary Solana Anchor framework utilities.
use anchor_lang::prelude::*;

/*
* Every Solana program (smart contract) has a unique public key assigned during deployment.
* This `declare_id!` macro is used to declare the program's public key. 
* It is a required entry point for Solana to link this code with the deployed program ID.
* 
* `declare_id!` will assign the program ID to this specific hardcoded value. 
* The program ID is a unique identifier for your deployed Solana program.
*/
declare_id!("HYvHAjNe27w6BbpnEKMvRVMs5mzHSru6Kr8AdNeqBnXc");

/*
* The `ANCHOR_DISCRIMINATOR_SIZE` is used to define the size of the Anchor discriminator.
* The discriminator is an 8-byte marker that helps Anchor differentiate between different account types. 
* It's necessary for efficient memory and account management.
*/
pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program] // Marks this module as the main Solana program. It's an entry point for Solana to interact with this Rust program.
pub mod favorites {
    use super::*; // Imports everything from the parent module.

    /*
    * This function allows users to set their favorite number, color, and hobbies.
    * It interacts with the Solana blockchain and the `Favorites` account.
    * The function uses the `Context<SetFavorites>` to access the account information.
    * It receives a number, a color, and a list of hobbies from the user to store in the `Favorites` account.
    *
    * The function returns `Ok(())` on success, indicating the transaction was successful.
    */
    pub fn set_favorites(
        context: Context<SetFavorites>, // The `Context` struct provides access to the required accounts.
        number: u64,                    // The user's favorite number, an unsigned 64-bit integer.
        color: String,                  // The user's favorite color, a string.
        hobbies: Vec<String>,           // The user's favorite hobbies, a vector of strings.
    ) -> Result<()> {
        // Log a message to Solana logs, showing the program's ID for debugging.
        msg!("Greetings from {}", context.program_id);

        // Extract the public key of the user from the context (the account that signed the transaction).
        let user_public_key = context.accounts.user.key();

        // Log the user's favorite number, color, and hobbies in a human-readable format.
        msg!(
            "User {}'s favorite number is {}, favorite color is {}, and their hobbies are {:?}",
            user_public_key,
            number,
            color,
            hobbies
        );

        // Store the user's favorites (number, color, and hobbies) in the `favorites` account.
        context.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        // Return success (the transaction has been completed).
        Ok(())
    }
}

/*
* The `Favorites` struct is a Solana account that stores the user's favorites.
* It holds the user's favorite number, color, and hobbies.
*/
#[account] // Marks this struct as a Solana account, making it a persistent storage.
#[derive(InitSpace)] // Automatically calculates and allocates the necessary space for the account.
pub struct Favorites {
    pub number: u64, // The favorite number, stored as an unsigned 64-bit integer (8 bytes).

    #[max_len(50)] // Enforces a maximum length of 50 characters for the color string.
    pub color: String, // The favorite color, stored as a string (with a max length of 50 characters).

    #[max_len(5, 50)] // Limits the hobbies array to a maximum of 5 items, each up to 50 characters.
    pub hobbies: Vec<String>, // A vector of strings, where each string represents a hobby.
}

/*
* The `SetFavorites` struct defines the relationships between the accounts required for the `set_favorites` function.
* It marks which accounts are used for the transaction, ensuring the correct permissions and actions.
*/
#[derive(Accounts)] // Marks this struct as defining account relationships for the `set_favorites` function.
pub struct SetFavorites<'info> {
    #[account(mut)] // The `user` account must be mutable (i.e., can be modified).
    pub user: Signer<'info>, // The user account that signs the transaction.

    /*
    * This defines the `favorites` account, which stores the favorite data.
    * If the account doesn't already exist, it will be created during the transaction.
    * The account creation is paid for by the user who initiates the transaction.
    * The `seeds` parameter is used to derive the account's public key deterministically.
    * The `bump` ensures the account is created with a unique bump seed.
    */
    #[account(
        init_if_needed, // Create the account if it doesn't exist.
        payer = user, // The user will pay for the account creation.
        space = ANCHOR_DISCRIMINATOR_SIZE + 8 + 50 + (5 * 50), // Space calculation.
        seeds = [b"favorites", user.key().as_ref()], // PDA derivation.
        bump // Auto-generated bump seed.
    )]
    pub favorites: Account<'info, Favorites>, // The account storing favorite data.

    // The system program is required for general transactions on Solana.
    pub system_program: Program<'info, System>, // Required system program for transactions.
}
