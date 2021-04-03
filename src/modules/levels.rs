use serenity::{
    framework::standard::{
        Args,
        CommandResult,
        macros::{command, group},
    },
    model::prelude::*,
    prelude::*
};

use crate::DATABASE;

#[group]
#[commands(rank)]
struct Levels;

#[command]
async fn rank(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

pub async fn on_message(ctx: Context, msg: Message) {
    println!("Message received: {}", msg.content);
}
