use serenity::{
    model::prelude::*, prelude::*
};
use std::time::Instant;
use tracing::debug;
use crate::DATABASE;
use crate::CONFIG;

pub async fn level_up(ctx: &Context, msg: &Message, target: Member, level: i32) {

    let now = Instant::now();
    let mut database = DATABASE.get().expect("Database not initialized").lock().await;

    let role_id = database.get_rank_reward(msg.guild_id.unwrap().to_string(), level).await
        .map(|r| RoleId::from(r));

    let level_up_message = database.get_config(
        msg.guild_id.unwrap().to_string(), "level_up_message").await
        .unwrap_or(CONFIG.get().unwrap().modules.ranks.default_level_up_message.clone());

    let send_level_up_message = database.get_config(
        msg.guild_id.unwrap().to_string(), "send_level_up_message")
        .await.as_deref() == Some("true");

    drop(database);
    debug!("Level up event locked database for {} micro seconds", now.elapsed().as_micros());

    // replace placeholders
    let level_up_message = level_up_message
        .replace("%player%", &msg.author.mention().to_string())
        .replace("%level%", &level.to_string());

    // send level up message if enabled
    if send_level_up_message {
        msg.channel_id.say(&ctx.http, level_up_message)
            .await.expect("Unable to send message");
    }

    // apply level up roles if applicable
    if let Some(role_id) = role_id {
        give_level_roles(ctx, target, role_id).await;
    }

}

async fn give_level_roles(ctx: &Context, mut target: Member, role_id: RoleId) {
    if target.add_role(&ctx.http, role_id).await.is_err() {
        debug!("Failed to add role to {}", target.user.name);
    }
}
