use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::command;
use tracing::error;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
        error!("Error while sending message: {:?}", why);
    }

    Ok(())
}