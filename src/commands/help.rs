use serenity::{framework::standard::Args, http::Http, prelude::*};
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use crate::CONFIG;

#[command]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match args.current() {
        Some("test") => {
            println!("args is test")
        }
        Some(_) => {
            help_main(&ctx.http, msg).await?;
        }
        None => {
            help_main(&ctx.http, msg).await?;
        }
    }

    Ok(())
}

async fn help_main(http: &Http, msg: &Message) -> CommandResult {

    msg.channel_id.send_message(http, |m| {
        m.embed(|e| {
            e.color(0xD3A6F6);
            e.title("**Help**");
            e.field("**Moderator**", format!("`{}help moderator`", CONFIG.prefix), true)
        })
    }).await?;

    Ok(())
}