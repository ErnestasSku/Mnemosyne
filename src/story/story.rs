use std::collections::HashSet;
use std::path::Path;
use std::sync::Arc;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::story::story_builder::map_stories_p;
use crate::story::story_structs::{StoryBlock, StoryContainer};

#[derive(Debug, Clone)]
pub struct StoryListener {
    story_name: String,
    current_story_path: Option<Arc<StoryBlock>>,
}

impl StoryListener {
    pub fn new(story: &Arc<StoryBlock>, story_name: &str) -> StoryListener {
        StoryListener {
            story_name: story_name.to_owned(),
            current_story_path: Some(story.clone()),
        }
    }
}

pub struct StoryListenerContainer;
pub struct LoadedStoryContainer;

#[command]
#[aliases("start", "begin")]
#[description = "Lets you start a story which was selected"]
async fn start_story(ctx: &Context, msg: &Message) -> CommandResult {
    let (user_lock, story_lock) = {
        let data_read = ctx.data.read().await;

        (
            data_read
                .get::<StoryListenerContainer>()
                .expect("Expected StoryListenerContainer in TypeMap")
                .clone(),
            data_read
                .get::<LoadedStoryContainer>()
                .expect("Expected LoadedStoryContainer in TypeMap")
                .clone(),
        )
    };

    {
        let story = story_lock.read().await;
        let story_block = story.as_ref().cloned();

        match story_block {
            None => {
                msg.reply(ctx, "No story found").await?;
            }
            Some((story, story_name)) => {
                let mut user_map = user_lock.write().await;

                let new_story = StoryListener::new(&story, &story_name);

                let content = new_story
                    .clone()
                    .current_story_path
                    .map(|x| x.present())
                    .expect("This should always have a value.");
                msg.reply(ctx, content).await?;

                user_map.insert(msg.author.id, new_story);
            }
        }
    }

    Ok(())
}

#[command]
#[aliases("action", "do")]
#[description = "You can make a choice"]
async fn action(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let command_name = args.single::<String>()?;

    println!("Action - {:?}", &command_name);

    if command_name.is_empty() {
        msg.reply(ctx, "invalid command").await?;
    } else {
        let user_lock = {
            let data_read = ctx.data.read().await;
            data_read
                .get::<StoryListenerContainer>()
                .expect("Expected StoryListenerContainer in TypeMap")
                .clone()
        };

        {
            let mut user_map = user_lock.write().await;
            let user = user_map.get(&msg.author.id);
            let mut new_user_value = None;
            match user {
                None => {
                    msg.reply(ctx, "User hasn't started the story yet").await?;
                }
                Some(user_prime) => {
                    println!("Inside user_prime block");
                    let mut index = -1;
                    let current_story = &user_prime.story_name.clone();
                    match &user_prime.current_story_path {
                        None => {
                            msg.reply(ctx, "No active story").await?;
                        }
                        Some(val) => {
                            //Another unwrap related to locks.
                            for (i, data) in val.path.lock().unwrap().iter().enumerate() {
                                if data.1 == command_name {
                                    index = i as i32;
                                }

                                println!("\n\n{:?}\n\n", data);
                            }

                            println!("Index - {index}");
                            if index == -1 {
                                new_user_value = None;
                            } else {
                                new_user_value = Some(StoryListener::new(
                                    //Another unwrap related to locks
                                    &val.path
                                        .lock()
                                        .unwrap()
                                        .get(index as usize)
                                        .expect("Story path should always have an index")
                                        .0,
                                    current_story,
                                ));
                            }
                        }
                    }
                }
            }

            if new_user_value.is_some() {
                let temp = new_user_value.clone();
                user_map.insert(
                    msg.author.id,
                    new_user_value.expect("is_some used, should never fail unwrapping"),
                );
                println!("{:?}", &temp);
                if let Some(st) = temp {
                    if let Some(message) = &st.current_story_path {
                        msg.reply(ctx, message.present()).await?;
                    }
                } else {
                    println!("It was None");
                }
            }
        }
    }

    Ok(())
}

#[command]
#[allowed_roles("Muse", "muse")]
#[description = "Loads a story file from computer into memory. Usage: ~story load C:\\User\\...\\story_name.story"]
async fn load(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    println!("{:?}", args);
    let file_path = args.single::<String>()?;

    println!("{:?}", &file_path);
    if file_path.is_empty() {
        msg.reply(
            ctx,
            "Error occurred during parsing of the command. (File path not supplied?)",
        )
        .await?;
        return Ok(());
    }

    let story = map_stories_p(&file_path);
    let file_name = Path::new(&file_path);
    let file_name = file_name
        .file_stem()
        .ok_or_else(|| String::from("Failed to get file stem"))?
        .to_str()
        .ok_or_else(|| String::from(""))
        .map(|x| x.to_string());

    match file_name {
        Ok(file_name) => {
            match story {
                Ok(story) => {
                    let story_lock = {
                        let data_read = ctx.data.read().await;
                        data_read
                            .get::<StoryContainer>()
                            .expect("Expected StoryContainer in TypeMap")
                            .clone()
                    };

                    {
                        let mut map = story_lock.write().await;
                        map.insert(file_name, story);
                    }
                }
                Err(err) => {
                    msg.reply(ctx, err).await?;
                    return Ok(());
                }
            }
            msg.reply(ctx, "Loaded successfully").await?;
        }
        Err(err) => {
            msg.reply(ctx, "Error happened").await?;
            println!("{}", err);
        }
    }
    Ok(())
}

#[command]
#[allowed_roles("Muse", "muse")]
#[description = "Prints a list of loaded stories."]
async fn read_loaded(ctx: &Context, msg: &Message) -> CommandResult {
    let story_lock = {
        let data_read = ctx.data.read().await;
        data_read
            .get::<StoryContainer>()
            .expect("Expected StoryContainer in TypeMap")
            .clone()
    };
    {
        let stories = story_lock.read().await.clone();

        println!("{:?}", &stories);
        let message = stories
            .into_keys()
            .collect::<Vec<String>>()
            .iter()
            .enumerate()
            .map(|(i, x)| i.to_string() + ". " + x + "\n")
            .collect::<Vec<String>>()
            .concat();

        msg.reply(ctx, message).await?;
    }

    Ok(())
}

#[command]
#[allowed_roles("Muse", "muse")]
#[description = "Selects a story to be played. Usage: ~story set_story storyName"]
async fn set_story(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let story_name = args.single::<String>()?;
    if story_name.is_empty() {
        msg.reply(
            ctx,
            "Error occurred during parsing of the command. (File path not supplied?)",
        )
        .await?;
        return Ok(());
    }

    let (curr_story_lock, story_lock) = {
        let data_read = ctx.data.read().await;

        (
            data_read
                .get::<LoadedStoryContainer>()
                .expect("Expected LoadedStoryContainer in TypeMap")
                .clone(),
            data_read
                .get::<StoryContainer>()
                .expect("Expected in TypeMap")
                .clone(),
        )
    };

    {
        let story_map = story_lock.read().await;
        let story = story_map.get(&story_name);

        match story {
            None => {
                msg.reply(ctx, "There is no such story").await?;
            }
            Some(story) => {
                let mut current_story = curr_story_lock.write().await;
                current_story.replace((story.clone(), story_name.clone()));

                msg.reply(ctx, "Loaded the story").await?;
            }
        }
    }

    Ok(())
}

#[command]
#[allowed_roles("Muse", "muse")]
#[description = "Clean up command. Used when story is no longer needed to be loaded in memory."]
async fn clear_story(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let story_name = args.single::<String>()?;

    if story_name.is_empty() {
        msg.reply(ctx, "No story name provided").await?;
        return Ok(());
    }

    let (story_lock, listener_lock, loaded_lock) = {
        let data_read = ctx.data.read().await;

        (
            data_read
                .get::<StoryContainer>()
                .expect("Expected StoryContainer in TypeMap")
                .clone(),
            data_read
                .get::<StoryListenerContainer>()
                .expect("Expected Storylistener in TypeMap")
                .clone(),
            data_read
                .get::<LoadedStoryContainer>()
                .expect("Expected LoadedStoryContainer in TypeMap")
                .clone(),
        )
    };

    {
        let mut story_map = story_lock.write().await;
        match story_map.remove(&story_name) {
            Some(story) => {
                let mut visited = HashSet::new();
                let mut to_cleanup = Vec::new();
                StoryBlock::story_to_list_unique(&story, &mut visited, &mut to_cleanup);

                // Clear story from all users.
                {
                    let mut listeners = listener_lock.write().await;
                    for (_, v) in listeners.iter_mut() {
                        if v.story_name == story_name {
                            v.current_story_path = None;
                        }
                    }
                }

                //If story is loaded, unload it.
                {
                    let mut loaded = loaded_lock.write().await;
                    let mut remove = false;
                    if let Some((_, name)) = &*loaded {
                        if *name == story_name {
                            remove = true;
                        }
                    }

                    if remove {
                        *loaded = None;
                    }
                }

                for unique in to_cleanup.iter_mut() {
                    *unique.path.lock().unwrap() = Vec::new();
                }

                drop(to_cleanup);
                drop(story);

                msg.react(ctx, '👍').await?
            }
            None => msg.react(ctx, '👎').await?,
        };
    }

    Ok(())
}

// async fn clear_all()
