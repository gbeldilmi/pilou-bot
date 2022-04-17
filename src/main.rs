use serenity::{
    async_trait,
    client::{
        Client,
        Context,
        EventHandler
    },
    model::{
        gateway::Ready,
        prelude::{
            channel::{
                Message
            },
        }
    }
};
use std::env;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    // Once the bot is ready
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
    // For every message
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.author.bot {
            if let Err(why) = msg.channel_id.say(&ctx, "Pilou !").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("PILOU_BOT_TOKEN").expect("Token");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
