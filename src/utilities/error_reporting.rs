use tracing::error;

use serenity::{framework::standard::CommandResult, model::prelude::Message, prelude::Context};

pub async fn bot_inform_command_error<ErrorMessage>(
    ctx: &Context,
    msg: &Message,
    error: ErrorMessage,
) -> CommandResult
where
    ErrorMessage: Into<String> + tracing::Value + std::marker::Copy,
{
    error!("{}", error.into());

    //TODO change this to be based on debug instead of hardcoded
    // if isDebug {
    if true {
        msg.reply(ctx, error.into()).await?;
    } else {
        msg.react(ctx, '❌').await?;
    }

    Ok(())
}
