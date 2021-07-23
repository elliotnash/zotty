use serenity::{
    model::prelude::*,
    prelude::*
};
use crate::commands::{Args, CommandResult};

use crate::CONFIG;

pub async fn help(ctx: Context, msg: Message, args: Args) {
    let prefix = CONFIG.get().unwrap().prefix.clone();
    match args.current() {
        Some("mod") | Some("moderator") => {
            help_moderator(&ctx, &msg, &prefix).await
        }
        Some("music") => {
            help_music(&ctx, &msg, &prefix).await
        }
        Some("utils") | Some("utilities") => {
            help_utilities(&ctx, &msg, &prefix).await
        }
        _ => {
            help_main(&ctx, &msg, &prefix).await
        }
    }.expect("Error dispatching help command");
}

pub async fn send_usage(ctx: &Context, msg: &Message, error: &str, usage: &str) {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.error);
            e.title(format!("**{}**", error));
            e.description(format!("Usage: `{0}{1}`", CONFIG.get().unwrap().prefix, usage))
        })
    }).await.expect("Failed to send message");
}
pub async fn send_error(ctx: &Context, msg: &Message, error: &str) {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.error);
            e.title("**Error**");
            e.description(error)
        })
    }).await.expect("Failed to send message");
}

pub fn get_hd_url(guild: &Guild) -> Option<String> {
    guild.icon_url().map(|mut s| {s.truncate(s.len()-5); s})
}

async fn help_main(ctx: &Context, msg: &Message, prefix: &str) -> CommandResult {

    let guild = msg.guild(&ctx).await.unwrap();
    let thumbnail_url = get_hd_url(&guild);

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.help);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help**");
            e.field("**Moderator**", format!("`{}help moderator`", prefix), true);
            e.field("**Music**", format!("`{}help music`", prefix), true);
            e.field("**Utilities**", format!("`{}help utilities`", prefix), true)
        })
    }).await?;

    Ok(())
}

async fn help_moderator(ctx: &Context, msg: &Message, prefix: &str) -> CommandResult {

    let guild = msg.guild(&ctx).await.unwrap();
    let thumbnail_url = get_hd_url(&guild);

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.moderator);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Moderator**");
            e.field(format!("`{}mute [user]`", prefix), "Mutes the mentioned user", false);
            e.field(format!("`{}kick [user]`", prefix), "Kicks the mentioned user", false);
            e.field(format!("`{}ban [user]`", prefix), "Bans the mentioned user", false);
            e.field(format!("`{}unmute [user]`", prefix), "Unmutes the mentioned user", false);
            e.field(format!("`{}purge [number of messages]`", prefix), "Clears a specified amount of messages", false);
            e.field(format!("`{}stream [message]`", prefix), "Sets the bot's status to streaming the specified message", false);
            e.field(format!("`{}setup [help | discord]`", prefix), 
                format!("Sets up the discord bot. See `{}setup help` for more information", prefix), false)
        })
    }).await?;

    Ok(())
}

async fn help_music(ctx: &Context, msg: &Message, prefix: &str) -> CommandResult {

    let guild = msg.guild(&ctx).await.unwrap();
    let thumbnail_url = get_hd_url(&guild);

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.music);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Music**");
            e.field(format!("`{}connect [user]`", prefix), format!("Joins the voice channel the user is in \n\
            *aliases:* `{}join`", prefix), false);
            e.field(format!("`{}play [name to search or url]`", prefix), format!("Plays the specified song \n\
            *aliases:* `{}p`", prefix), false);
            e.field(format!("`{}pause`", prefix), "Pauses the current song", false);
            e.field(format!("`{}resume`", prefix), "Resumes the current song", false);
            e.field(format!("`{}skip`", prefix), format!("Skips the current song \n\
            *aliases:* `{}s`", prefix), false);
            e.field(format!("`{}queue`", prefix), format!("Shows which songs are in the queue \n\
            *aliases:* `{0}playlist, {0}q`", prefix), false);
            e.field(format!("`{}playing`", prefix), format!("Lists the current song \n\
            *aliases:* `{0}current, {0}np`", prefix), false);
            e.field(format!("`{}volume [volume 1-100]`", prefix), format!("Adjusts the volume for all users \n\
            *aliases:* `{}vol`", prefix), false);
            e.field(format!("`{}stop`", prefix), "Stops the song and disconnects from the voice channel. \n\
            this will clear the entire playlist", false)
        })
    }).await?;

    Ok(())
}

async fn help_utilities(ctx: &Context, msg: &Message, prefix: &str) -> CommandResult {

    let guild = msg.guild(&ctx).await.unwrap();
    let thumbnail_url = get_hd_url(&guild);

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.commands);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Utilities**");
            e.field(format!("`{}say [message]`", prefix), "Repeats what the user says", false);
            e.field(format!("`{}poll [message]`", prefix), "Creates a poll", false);
            e.field(format!("`{}suggest [message]`", prefix), "Suggests an idea to #server-suggestions", false)
        })
    }).await?;

    Ok(())
}
