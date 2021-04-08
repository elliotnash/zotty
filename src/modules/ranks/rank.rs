use super::rank_card::generate_rank_card;
use serenity::{
    model::prelude::*, prelude::*
};
use crate::commands::Args;
use tracing::debug;
use std::time::Instant;

use crate::DATABASE;
use super::help;


pub async fn rank(ctx: Context, msg: Message, args: Args) {

    debug!("Ranks command is firing");
    
    //Don't allow in dms
    let guild_id = if msg.guild_id.is_none() {
        help::send_error(&ctx, &msg, "Sorry, you can't use that command here").await;
        return;
    } else {msg.guild_id.unwrap()};

    // get the userid of the target
    let target_id = if args.is_empty() {
        Some(msg.author.id)
    } else if args.len() == 1 {
        Some(args.current().unwrap().parse::<UserId>().unwrap())
    } else {
        None
    };

    // get option of target member
    let target = if let Some(target_id) = target_id {
        guild_id.member(&ctx, target_id).await.ok()
    } else {
        None
    };

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
        
        let db_user;
        let rank;
        {
            let now = Instant::now();
            let mut database = DATABASE.get().expect("Database not initialized").lock().await;
            debug!("Rank command got lock on database in {} micro seconds", now.elapsed().as_micros());
            db_user = database.get_user(guild_id.to_string(), target.user.id.to_string()).await;
            rank = database.get_rank(guild_id.to_string(), &db_user).await;
            drop(database);
        }
        let now = Instant::now();
        let writer = generate_rank_card(target.user, db_user.clone(), rank).await;
        debug!("generating rank card took {}ms", now.elapsed().as_millis());

        msg.channel_id.send_files(&ctx.http, vec![(writer.buffer(), "rank.png")], |m| {m}).await
            .expect("Failed to send message");

    }
}
