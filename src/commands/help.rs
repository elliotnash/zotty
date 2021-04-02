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
            help_main(ctx, msg).await?;
        }
        None => {
            help_main(ctx, msg).await?;
        }
    }

    Ok(())
}

async fn help_main(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(0xD3A6F6);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help**");
            e.field("**Moderator**", format!("`{}help moderator`", CONFIG.prefix), true)
        })
    }).await?;

    Ok(())
}