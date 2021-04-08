use chrono::Utc;
use serenity::{
    cache::FromStrAndCache,
    model::prelude::*, prelude::*
};
use tracing::debug;
use rand::Rng;
use std::time::Instant;

use crate::DATABASE;
use super::help;

mod colour;

pub mod rank;
pub mod leaderboard;

pub fn get_level_xp(level: i32) -> i32 {
    5 * level.pow(2) + 50 * level + 100
}

pub async fn on_message(ctx: Context, msg: Message) {

    //Don't award points to bots
    if msg.author.bot {return;};

    let guild_id = if let Some(guild_id) = msg.guild_id {guild_id} else {return;};

    let target = if let Ok(target) = 
        guild_id.member(&ctx, msg.author.id).await {
            target
        } 
    else {return;};

    let has_tester_role = if let Some(roles) = target.roles(&ctx).await {
        let rolenames: Vec<String> = roles.iter().map(|role| role.name.to_lowercase()).collect();
        rolenames.contains(&"tester".to_string())
    } 
    else {false};
    

    if has_tester_role {
        let now = Instant::now();
        let mut database = DATABASE.get().expect("Database not initialized").lock().await;

        let db_user = database.get_user(guild_id.to_string(), msg.author.id.to_string()).await;
        
        // only award if user hasn't been awarded in the last minute
        if (Utc::now() - db_user.last_xp).num_seconds() > 59 {

            let xp = db_user.xp + rand::thread_rng().gen_range(15..25);
            let level_xp = get_level_xp(db_user.level);

            if xp > level_xp {
                database.set_user_level(guild_id.to_string(), msg.author.id.to_string(),
                    db_user.level+1, xp-level_xp).await;
                drop(database);
                level_up(&ctx, &msg, db_user.level+1).await;
            } else {
                database.set_user_xp(guild_id.to_string(), msg.author.id.to_string(),
                    xp).await;
            }
        }
        debug!("Message event locked database for {} micro seconds", now.elapsed().as_micros());

    }

}

async fn level_up(ctx: &Context, msg: &Message, level: i32) {
    msg.channel_id.say(&ctx.http, format!("GG {0}, you just advanced to level {1}!",
        msg.author.mention().to_string(), level))
        .await.expect("Unable to send message");
}
