use crate::{Data, Error};

#[poise::command(slash_command)]
pub async fn test(context: poise::Context<'_, Data, Error>) -> Result<(), Error> {
    let user = &context.author().name;
    context
        .send(|builder| {
            builder
                .embed(|embed| {
                    embed
                        .author(|author| {
                            author
                                .name("Test author")
                                .url("https://twitter.com/ffffff_256")
                        })
                        .field("Test field", "Hello!", false)
                })
                .ephemeral(false)
        })
        .await
        .expect("Failed to send reply");

    println!("{user}");
    Ok(())
}
