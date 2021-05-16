//
// file: main.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
extern crate serenity;

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use std::env;


struct Handler;

//
// Implementation for the event handler structure for used to
// handle Discord bot events.
//
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, _: Context, msg: Message) {
        if msg.content == "!help" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say("Noone is gonna help you rusty boi!") {
                println!("Error sending message: {:?}", why);
            } // end if
        } // end if

        if msg.content == "!messageme" {
            // If the `methods` feature is enabled, then model structs will
            // have a lot of useful methods implemented, to avoid using an
            // often otherwise bulky Context, or even much lower-level `rest`
            // method.
            //
            // In this case, you can direct message a User directly by simply
            // calling a method on its instance, with the content of the
            // message.
            if let Err(why) = msg.author.dm(|m| m.content("Hello!")) {
                println!("Error when direct messaging user: {:?}", why);
            } // end if
        } // end if
    } // end of function message

    //
    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    //
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    } // end of function ready

} // end impl


//
// main is where program execution starts
//
fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
