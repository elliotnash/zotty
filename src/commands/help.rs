use serenity::{framework::standard::Args, prelude::*};
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
        Some("music") => {
            help_music(ctx, msg).await?;
        }
        Some("cmd") | Some("commands") => {
            help_commands(ctx, msg).await?;
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
#[aliases("cmd")]
async fn commands(ctx: &Context, msg: &Message) -> CommandResult {
    help_commands(ctx, msg).await?;
    Ok(())
}

async fn help_main(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.colours.help);
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
            e.color(CONFIG.colours.moderator);
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
            e.color(CONFIG.colours.music);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Music**");
            e.field(format!("`{}connect [user]`", CONFIG.prefix), format!("Joins the voice channel the user is in\
            *aliases:* `{}join`", CONFIG.prefix), false);
            e.field(format!("`{}play [name to search or url]`", CONFIG.prefix), format!("Plays the specified song\
            *aliases:* `{}p`", CONFIG.prefix), false);
            e.field(format!("`{}pause`", CONFIG.prefix), "Pauses the current song", false);
            e.field(format!("`{}resume`", CONFIG.prefix), "Resumes the current song", false);
            e.field(format!("`{}skip`", CONFIG.prefix), format!("Skips the current song\
            *aliases:* `{}s`", CONFIG.prefix), false);
            e.field(format!("`{}queue`", CONFIG.prefix), format!("Shows which songs are in the queue\
            *aliases:* `{0}playlist, {0}q`", CONFIG.prefix), false);
            e.field(format!("`{}playing`", CONFIG.prefix), format!("Lists the current song\
            *aliases:* `{0}current, {0}np`", CONFIG.prefix), false);
            e.field(format!("`{}volume [volume 1-100]`", CONFIG.prefix), format!("Adjusts the volume for all users\
            *aliases:* `{}vol`", CONFIG.prefix), false);
            e.field(format!("`{}stop`", CONFIG.prefix), "Stops the song and disconnects from the voice channel.\
            this will clear the entire playlist", false)
        })
    }).await?;

    Ok(())
}

async fn help_commands(ctx: &Context, msg: &Message) -> CommandResult {

    let thumbnail_url = msg.guild(&ctx.cache).await.unwrap().icon_url();

    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.color(CONFIG.colours.commands);
            if let Some(url) = thumbnail_url {e.thumbnail(url);};
            e.title("**Help Commands**");
            e.field(format!("`{}say [message]`", CONFIG.prefix), "Repeats what the user says", false);
            e.field(format!("`{}poll [message]`", CONFIG.prefix), "Creates a poll", false);
            e.field(format!("`{}suggest [message]`", CONFIG.prefix), "Suggests an idea to #server-suggestions", false)
        })
    }).await?;

    Ok(())
}
