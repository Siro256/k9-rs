use crate::commands::test::test;
use crate::config::CONFIG;
use poise::serenity_prelude as serenity;

mod commands;
mod config;
mod event_handler;

type Error = Box<dyn std::error::Error + Send + Sync>;

pub struct Data {}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .token(&CONFIG.discord_token)
        .intents(serenity::GatewayIntents::empty())
        .options(poise::FrameworkOptions {
            commands: vec![test()],
            ..Default::default()
        })
        .setup(move |context, _, framework| {
            Box::pin(async move {
                println!("Ready!");
                poise::builtins::register_globally(context, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    tokio::spawn(async move {
        let _: Result<_, _> = framework
            .run()
            .await
            .map_err(|reason| eprintln!("Client stopped: ${reason:?}"));
    });

    tokio::signal::ctrl_c().await.unwrap();

    println!("Shutdown")
}
