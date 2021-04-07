use std::{sync::Arc, time::Instant};
use tokio::sync::Mutex;
use tokio::task;
use once_cell::sync::OnceCell;

use serenity::{async_trait, client::bridge::gateway::ShardManager, model::{channel::Message, gateway::Ready}, prelude::*};
use tracing::{error, info, debug};

mod commands;
mod modules;
use modules::{
    help,
    ranks
};
mod config;
use config::Config;
mod database;
use database::Database;

static CONFIG: OnceCell<Config> = OnceCell::new();
static DATABASE: OnceCell<Arc<Mutex<Box<dyn Database>>>> = OnceCell::new();

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
    async fn message(&self, ctx: Context, msg: Message) {

        //dispatch message event to modules that need it
        task::spawn(ranks::on_message(ctx.clone(), msg.clone()));

    }
}

//init client
#[tokio::main]
async fn main() {

    // initialize logger
    tracing_subscriber::fmt::init();

    //initialize config
    CONFIG.set(Config::from_file()).expect("Failed to load config");
    //initialize database
    DATABASE.set(Arc::new(Mutex::new(database::new_database().await))).expect("Unable to connect to database");

    let mut client = Client::builder(&CONFIG.get().unwrap().token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        println!();
        info!("ETechBot is shutting down");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }

}
