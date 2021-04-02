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
        Some("mod") | Some("moderator") => {
            help_moderator(ctx, msg).await?;
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
            e.field("**Moderator**", format!("`{}help moderator`", CONFIG.prefix), true);
            e.field("**Music**", format!("`{}help music`", CONFIG.prefix), true);
            e.field("**Commands**", format!("`{}help commands`", CONFIG.prefix), true)
        })
    }).await?;

    Ok(())
}

async fn help_moderator(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(0xF05101);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Moderator**");
            e.field(format!("`{}mute [user]`", CONFIG.prefix), "Mutes the mentioned user", false);
            e.field(format!("`{}kick [user]`", CONFIG.prefix), "Kicks the mentioned user", false);
            e.field(format!("`{}ban [user]`", CONFIG.prefix), "Bans the mentioned user", false);
            e.field(format!("`{}unmute [user]`", CONFIG.prefix), "Unmutes the mentioned user", false);
            e.field(format!("`{}purge [number of messages]`", CONFIG.prefix), "Clears a specified amount of messages", false);
            e.field(format!("`{}stream [message]`", CONFIG.prefix), "Sets the bot's status to streaming the specified message", false);
            e.field(format!("`{}setup [help | discord]`", CONFIG.prefix), 
                format!("Sets up the discord bot. See `{}setup help` for more information", CONFIG.prefix), false)
        })
    }).await?;

    Ok(())
}

async fn help_music(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(0xD3A6F6);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help**");
            e.field("**Moderator**", format!("`{}help moderator`", CONFIG.prefix), true);
            e.field("**Music**", format!("`{}help music`", CONFIG.prefix), true);
            e.field("**Commands**", format!("`{}help commands`", CONFIG.prefix), true)
        })
    }).await?;

    Ok(())
}

async fn help_commands(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(0xD3A6F6);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help**");
            e.field("**Moderator**", format!("`{}help moderator`", CONFIG.prefix), true);
            e.field("**Music**", format!("`{}help music`", CONFIG.prefix), true);
            e.field("**Commands**", format!("`{}help commands`", CONFIG.prefix), true)
        })
    }).await?;

    Ok(())
}
