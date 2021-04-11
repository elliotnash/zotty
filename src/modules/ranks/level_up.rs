use serenity::{
    model::prelude::*, prelude::*
};
use std::time::Instant;
use tracing::debug;
use crate::DATABASE;

pub async fn level_up(ctx: &Context, msg: &Message, level: i32) {

    let now = Instant::now();
    let mut database = DATABASE.get().expect("Database not initialized").lock().await;

    let role_id = database.get_rank_reward(msg.guild_id.unwrap().to_string(), level).await;
    if let Some(role_id) = role_id {
        let role_id = RoleId::from(role_id);
        dbg!(role_id);
    } else {
        debug!("Level up event found no rank rewards for this level");
    }

    debug!("Level up event locked database for {} micro seconds", now.elapsed().as_micros());


    msg.channel_id.say(&ctx.http, format!("GG {0}, you just advanced to level {1}!",
        msg.author.mention().to_string(), level))
        .await.expect("Unable to send message");
}
