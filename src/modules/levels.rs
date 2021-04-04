use chrono::{DateTime, Utc};
use serenity::{
    cache::FromStrAndCache, framework::standard::{
        Args,
        CommandResult,
        macros::{command, group},
    }, 
    model::prelude::*, prelude::*
};
use rand::{Rng, distributions::Uniform};

use crate::DATABASE;

#[group]
#[commands(rank)]
struct Levels;

#[command]
async fn rank(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    Ok(())
}

pub async fn on_message(ctx: Context, msg: Message) {

    let guild_id = if msg.guild_id.is_none() {return;} else {msg.guild_id.unwrap()};

    let role_id = RoleId::from_str(&ctx.cache, "827946575136948226").await;
    let role_id = if role_id.is_err() {return;} else {role_id.unwrap()};

    let has_tester_role = msg.author.has_role(&ctx.http, guild_id, role_id).await;
    if has_tester_role.is_err() {return;};
    let has_tester_role = has_tester_role.unwrap();

    if has_tester_role {
        let mut database = DATABASE.get().expect("Database not initialized").lock().await;
        let db_user = database.get_user(guild_id.to_string(), msg.author.id.to_string()).await;
        dbg!(&db_user);
        // only award if user hasn't been awarded in the last minute
        if (Utc::now() - db_user.last_xp).num_seconds() > 59 {
            println!("User should be getting awarded");
            let xp = rand::thread_rng().gen_range(15..25);
            database.set_user_xp(guild_id.to_string(), msg.author.id.to_string(),
                db_user.xp + xp).await;
        }

    }

}
