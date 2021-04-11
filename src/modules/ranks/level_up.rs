use serenity::{
    model::prelude::*, prelude::*
};
use std::time::Instant;
use tracing::debug;
use crate::DATABASE;

pub async fn level_up(ctx: &Context, msg: &Message, level: i32) {

    let now = Instant::now();
    let mut database = DATABASE.get().expect("Database not initialized").lock().await;

    let role_id = database.get_rank_reward(msg.guild_id.unwrap().to_string(), level).await
        .map(|r| RoleId::from(r));

    let level_up_message = database.get_config(
        msg.guild_id.unwrap().to_string(), "level_up_message").await
        .unwrap_or("GG %player%, you just advanced to level %level%".to_string());

    drop(database);
    debug!("Level up event locked database for {} micro seconds", now.elapsed().as_micros());

    // replace placeholders
    let level_up_message = level_up_message
        .replace("%player%", &msg.author.mention().to_string())
        .replace("%level%", &level.to_string());

    // send level up message
    msg.channel_id.say(&ctx.http, level_up_message)
        .await.expect("Unable to send message");
}
