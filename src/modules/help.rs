use serenity::{
    framework::standard::{
        Args,
        CommandResult,
        macros::{command, group},
    },
    model::prelude::*,
    prelude::*
};

use crate::CONFIG;

#[group]
#[commands(help, moderator, music, utilities)]
struct Help;

#[command]
async fn help(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    match args.current() {
        Some("mod") | Some("moderator") => {
            help_moderator(ctx, msg).await?;
        }
        Some("music") => {
            help_music(ctx, msg).await?;
        }
        Some("utils") | Some("utilities") => {
            help_utilities(ctx, msg).await?;
        }
        _ => {
            help_main(ctx, msg).await?;
        }
    }

    Ok(())
}

#[command]
#[aliases("mod")]
async fn moderator(ctx: &Context, msg: &Message) -> CommandResult {
    help_moderator(ctx, msg).await?;
    Ok(())
}
#[command]
async fn music(ctx: &Context, msg: &Message) -> CommandResult {
    help_music(ctx, msg).await?;
    Ok(())
}
#[command]
#[aliases("utils")]
async fn utilities(ctx: &Context, msg: &Message) -> CommandResult {
    help_utilities(ctx, msg).await?;
    Ok(())
}

pub async fn send_usage(ctx: &Context, msg: &Message, error: &str, usage: &str) {
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.error);
            e.title("**Error**");
            e.description(format!("{0}\nUsage: `{1}{2}`", error, CONFIG.get().unwrap().prefix, usage))
        })
    }).await.expect("Failed to send message");
}

async fn help_main(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.help);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help**");
            e.field("**Moderator**", format!("`{}help moderator`", CONFIG.get().unwrap().prefix), true);
            e.field("**Music**", format!("`{}help music`", CONFIG.get().unwrap().prefix), true);
            e.field("**Utilities**", format!("`{}help utilities`", CONFIG.get().unwrap().prefix), true)
        })
    }).await?;

    Ok(())
}

async fn help_moderator(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.moderator);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Moderator**");
            e.field(format!("`{}mute [user]`", CONFIG.get().unwrap().prefix), "Mutes the mentioned user", false);
            e.field(format!("`{}kick [user]`", CONFIG.get().unwrap().prefix), "Kicks the mentioned user", false);
            e.field(format!("`{}ban [user]`", CONFIG.get().unwrap().prefix), "Bans the mentioned user", false);
            e.field(format!("`{}unmute [user]`", CONFIG.get().unwrap().prefix), "Unmutes the mentioned user", false);
            e.field(format!("`{}purge [number of messages]`", CONFIG.get().unwrap().prefix), "Clears a specified amount of messages", false);
            e.field(format!("`{}stream [message]`", CONFIG.get().unwrap().prefix), "Sets the bot's status to streaming the specified message", false);
            e.field(format!("`{}setup [help | discord]`", CONFIG.get().unwrap().prefix), 
                format!("Sets up the discord bot. See `{}setup help` for more information", CONFIG.get().unwrap().prefix), false)
        })
    }).await?;

    Ok(())
}

async fn help_music(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.music);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Music**");
            e.field(format!("`{}connect [user]`", CONFIG.get().unwrap().prefix), format!("Joins the voice channel the user is in \n\
            *aliases:* `{}join`", CONFIG.get().unwrap().prefix), false);
            e.field(format!("`{}play [name to search or url]`", CONFIG.get().unwrap().prefix), format!("Plays the specified song \n\
            *aliases:* `{}p`", CONFIG.get().unwrap().prefix), false);
            e.field(format!("`{}pause`", CONFIG.get().unwrap().prefix), "Pauses the current song", false);
            e.field(format!("`{}resume`", CONFIG.get().unwrap().prefix), "Resumes the current song", false);
            e.field(format!("`{}skip`", CONFIG.get().unwrap().prefix), format!("Skips the current song \n\
            *aliases:* `{}s`", CONFIG.get().unwrap().prefix), false);
            e.field(format!("`{}queue`", CONFIG.get().unwrap().prefix), format!("Shows which songs are in the queue \n\
            *aliases:* `{0}playlist, {0}q`", CONFIG.get().unwrap().prefix), false);
            e.field(format!("`{}playing`", CONFIG.get().unwrap().prefix), format!("Lists the current song \n\
            *aliases:* `{0}current, {0}np`", CONFIG.get().unwrap().prefix), false);
            e.field(format!("`{}volume [volume 1-100]`", CONFIG.get().unwrap().prefix), format!("Adjusts the volume for all users \n\
            *aliases:* `{}vol`", CONFIG.get().unwrap().prefix), false);
            e.field(format!("`{}stop`", CONFIG.get().unwrap().prefix), "Stops the song and disconnects from the voice channel. \n\
            this will clear the entire playlist", false)
        })
    }).await?;

    Ok(())
}

async fn help_utilities(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.get().unwrap().colours.commands);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Utilities**");
            e.field(format!("`{}say [message]`", CONFIG.get().unwrap().prefix), "Repeats what the user says", false);
            e.field(format!("`{}poll [message]`", CONFIG.get().unwrap().prefix), "Creates a poll", false);
            e.field(format!("`{}suggest [message]`", CONFIG.get().unwrap().prefix), "Suggests an idea to #server-suggestions", false)
        })
    }).await?;

    Ok(())
}
