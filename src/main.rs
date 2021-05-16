//
// file: main.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
use bot::Bot;
use std::env;

mod bot;

//
// main is where program execution starts
//
#[tokio::main]
async fn main() {
    // We also want to option to store the token in a `.env` file
    dotenv::dotenv().ok();

    // Grab the bot token from the environment
    let token = env::var("{{token_variable_name}}").expect("Could not load token from environment");

    // Run the bot. If it errors out, let the user know.
    if let Err(_) = Bot::run(token.as_str()).await{
        println!("The bot encountered a catastrophic error. Please contact the bot creator if this happens frequently");
        return;
    }
} // end of function main
