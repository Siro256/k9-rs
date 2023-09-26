use serenity::async_trait;
use serenity::client::Context;
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!("Ready to work!")
    }
}
