//
// file: main.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
extern crate program;
use program::foundation;

use std::env;

#[group]
#[commands(ping)]
struct General;
struct Handler;

#[async_trait]
impl EventHandler for Handler {}

//
// main is where program execution starts
//
#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    } // end if

} // end of function main

//
// this function will send a message.
//
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
} // end of function ping
