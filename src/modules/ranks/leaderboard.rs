use serenity::{
    model::prelude::*, prelude::*
};
use std::time::Instant;
use std::str::FromStr;
use crate::commands::Args;
use tracing::debug;
use super::help;

use crate::DATABASE;

pub async fn leaderboard(ctx: Context, msg: Message, args: Args) {

    debug!("leaderboard command is firing");
    
    //Don't allow in dms
    let guild_id = if msg.guild_id.is_none() {
        help::send_error(&ctx, &msg, "Sorry, you can't use that command here").await;
        return;
    } else {msg.guild_id.unwrap()};

    // get option of target member
    let target = guild_id.member(&ctx, msg.author.id).await.ok();

    // send invalid usage if member isn't existant
    let target = if let Some(target) = target {
        target
    } else {
        help::send_usage(&ctx, &msg, "Invalid arguments", "rank [user]").await;
        return;
    };

    //Don't let target be bot
    if target.user.bot {
        help::send_error(&ctx, &msg, "Sorry, you can't use this command on a bot").await;
        return;
    };

    let has_tester_role = if let Some(roles) = target.roles(&ctx).await {
        let rolenames: Vec<String> = roles.iter().map(|role| role.name.to_lowercase()).collect();
        rolenames.contains(&"tester".to_string())
    } else {
        false
    };

    if has_tester_role {

        let mut page_num: i32 = FromStr::from_str(args.current().unwrap_or("1")).unwrap_or(1)-1;
        if page_num <= 0 {page_num = 0};

        let now = Instant::now();
        let db_users = {
            let mut database = DATABASE.get().expect("Database not initialized").lock().await;
            database.get_top_users(guild_id.to_string(), 10, 10*page_num).await
        };
        debug!("Leaderboard command took {} micro seconds to query database", now.elapsed().as_micros());

        dbg!(db_users);

    }
}
