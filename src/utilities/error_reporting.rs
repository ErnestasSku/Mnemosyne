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

    msg.react(ctx, '❌').await?;

    Ok(())
}
