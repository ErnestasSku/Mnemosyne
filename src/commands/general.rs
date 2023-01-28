

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
pub async fn info(ctx: &Context, msg: &Message) -> CommandResult {

    let help_message = "This is a bot created for story telling.
    
    General usage (note, in this example the prefix used is \"~\"):
    * To start a story type - ~story start OR ~story begin
    * To respond write ~story do word OR ~do word where word is the first word at the bottom of the text.
    
    
    Usage for moderators: 
    * To load a story into the memory type: ~story load C:\\file\\path\\to\\story\\file.story
    * To select a story type ~story set_story storyName
    
    To learn more about the bot or story files check out the repository at https://github.com/ErnestasSku/Mnemosyne";

    msg.channel_id.say(&ctx.http, help_message).await?;

    Ok(())
}