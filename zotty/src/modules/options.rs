use serenity::{
    model::prelude::*, prelude::*
};
use super::help;
use crate::{DATABASE, commands::Args};

pub async fn config(ctx: Context, msg: Message, args: Args) {

    //Don't allow in dms
    let guild_id = if msg.guild_id.is_none() {
        help::send_error(&ctx, &msg, "Sorry, you can't use that command here").await;
        return;
    } else {msg.guild_id.unwrap()};

    //Don't award points to bots
    if msg.author.bot {return;};

    let target = if let Ok(target) = 
        guild_id.member(&ctx, msg.author.id).await {
            target
        } 
    else {return;};

    let has_admin = target.permissions(&ctx).await.unwrap().administrator();
    

    if has_admin {
        let _database = DATABASE.get().expect("Database not initialized").lock().await;
        
        match args.current().unwrap_or("") {
            "set" => {

            }
            "get" => {
                
            }
            _ => {
                help::send_usage(&ctx, &msg, "Invalid arguments", "config {get|set} <key> [value]").await;
            }
        }

    }

}
