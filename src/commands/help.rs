use serenity::{framework::standard::Args, prelude::*};
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

#[command]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match args.current() {
        Some("test") => {
            println!("args is test")
        }
        Some(_) => {
            println!("Unkown arg")
        }
        None => {
            println!("no args")
        }
    }
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}