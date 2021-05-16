//
// file: bot.rs
// author: Michael Brockus
// gmail: <michaelbrockus@gmail.com>
//
use serenity::{
    Client, 
    client::Context,
    model::prelude::{Message, UserId},
    framework::standard::{
        help_commands, macros::{help, command, group}, Args, CommandGroup, CommandResult, HelpOptions, StandardFramework
    }
};
use std::collections::HashSet;

pub struct Bot;

impl Bot{
    pub async fn run(token: &str) -> serenity::Result<()>{
        // Create the standard framework
        let framework = StandardFramework::new()
            .configure(|c| {
                c.prefix("{{bot_prefix}}").case_insensitivity(true)
            })
            // Add each command group here using .group([GROUP NAME IN CAPS])
            {% if include_examples %}
            .group(&PONG_GROUP)
            {% endif %}
            .help(&HELP);

        // Create and start the client
        let mut client = Client::builder(token).framework(framework).await?;
        client.start().await
    }
}

#[help]
/// Generates help text for the bot
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
{% if include_examples %}
// This is an example group. You should put actual groups into their own separate files
#[group]
#[commands(pong)]
struct Pong;

#[command("ping")]
async fn pong(ctx: &Context, msg: &Message) -> CommandResult{
    msg.channel_id.broadcast_typing(ctx).await?;
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
{% endif %}
