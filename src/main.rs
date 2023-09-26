use serenity::model::gateway::GatewayIntents;
use crate::config::CONFIG;
use crate::event_handler::Handler;

mod config;
mod event_handler;

#[tokio::main]
async fn main() {
    let mut client =
        serenity::Client::builder(&CONFIG.discord_token, GatewayIntents::empty())
            .event_handler(Handler)
            .await
            .expect("Failed to create client");

    tokio::spawn(async move {
        let _: Result<_, _> =
            client.start()
                .await
                .map_err(|reason| eprintln!("Client stopped: {reason:?}"));
    });

    tokio::signal::ctrl_c().await.unwrap();

    println!("Shutdown")
}
