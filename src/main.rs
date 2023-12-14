mod commands;

use poise::serenity_prelude as serenity;
use poise::FrameworkError::*;
use dotenv::dotenv;
use std::env;

use commands::ping as ping;

// User data
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


// Error handler
async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        Setup { error, ..} => panic!("Failed to start bot: {:?}", error),
        Command { error, ctx, .. } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        },
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {:?}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Loads env file
    dotenv().ok();

    // Defines framework options
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping::ping()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            pre_command: |ctx| {
                Box::pin(async move {
                    println!("[LOG] Executing command {}...", ctx.command().qualified_name);
                })
            },
            // This code is run after a command if it was successful (returned Ok)
            post_command: |ctx| {
                Box::pin(async move {
                    println!("[LOG] Executed command {}!", ctx.command().qualified_name);
                })
            },
            ..Default::default()
        })
        .token(env::var("BOT_TOKEN").expect("missing BOT_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}