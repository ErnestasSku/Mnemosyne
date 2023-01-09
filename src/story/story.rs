use std::path::Path;
use std::sync::Arc;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::RwLock;

use crate::story::story2::{StoryContainer2, StoryBlock2};
use crate::story::story_builder::map_stories;



pub struct StoryListener {
    _current_story_path: Arc<StoryBlock2>
}

pub struct StoryListenerContainer;
impl TypeMapKey for StoryListenerContainer {
    type Value = Arc<RwLock<std::collections::HashMap<UserId, StoryListener>>>;
}



#[command]
#[aliases("start-story", "begin-story")]
async fn start_story(_ctx: &Context, _msg: &Message) -> CommandResult {

    Ok(())
}

#[command]
async fn load(ctx: &Context, msg: &Message) -> CommandResult {

    let file_path = &msg.content;
    let story = map_stories(file_path);
    let file_name = Path::new(file_path);
    let file_name = file_name.file_name().unwrap().to_str().unwrap().to_string();

    match story {
        Ok(story) => {
            let story_lock = {
                let data_read = ctx.data.read().await;
                data_read.get::<StoryContainer2>().expect("Expected StoryContainer2 in TypeMap").clone()
            };
            {
                let mut stories = story_lock.write().await;
                stories.insert(file_name, story);
            }
            
        },
        Err(err) => {
            msg.reply(ctx, err).await?;
            return Ok(());
        }
    }

    Ok(())
}

#[command]
async fn read_loaded(_ctx: &Context, _msg: &Message) -> CommandResult {



    Ok(())
}

#[command]
async fn set_story(_ctx: &Context, _msg: &Message) -> CommandResult {

    Ok(())
}