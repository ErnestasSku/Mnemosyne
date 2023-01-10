use std::path::Path;
use std::sync::Arc;

use serenity::framework::standard::macros::command;
use serenity::framework::standard::{CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tokio::sync::RwLock;

use crate::story::story2::{StoryContainer2, StoryBlock2};
use crate::story::story_builder::map_stories;


#[derive(Debug)]
pub struct StoryListener {
    current_story_path: Option<Arc<StoryBlock2>>
}

impl StoryListener {
    pub fn new(story: &Arc<StoryBlock2>) -> StoryListener {
        StoryListener { current_story_path: Some(story.clone()) }
    }
}

pub struct StoryListenerContainer;
pub struct LoadedStoryContainer;

impl TypeMapKey for StoryListenerContainer {
    type Value = Arc<RwLock<std::collections::HashMap<UserId, StoryListener>>>;
}

impl TypeMapKey for LoadedStoryContainer {
    // type Value = Arc<RwLock<Arc<Option<StoryBlock2>>>>;
    type Value = Arc<RwLock<Option<Arc<StoryBlock2>>>>;
}



#[command]
#[aliases("start-story", "begin-story")]
async fn start_story(ctx: &Context, msg: &Message) -> CommandResult {

    let (user_lock, story_lock) = {
        let data_read = ctx.data.read().await;

        (data_read.get::<StoryListenerContainer>().expect("Expected StoryListenerContainer in TypeMap").clone(),
        data_read.get::<LoadedStoryContainer>().expect("Expected LoadedStoryContainer in TypeMap").clone())
    };

    {
        let story = story_lock.read().await;
        let story_block = story.as_ref().and_then(|x| Some(x.clone()));

        match story_block {
            None => {
                msg.reply(ctx, "No story found").await?;
            },
            Some(story) => {
                let mut user_map = user_lock.write().await;
                
                //FIXME: This fails the first time for some reason
                let insert = user_map.insert(msg.author.id, StoryListener::new(&story));
                
                println!("{:?}", &insert);
                //Possibly create a new thread and start the story there
                let insert = insert.unwrap();
                let content = insert.current_story_path.and_then(|x| Some(x.present())).unwrap();
                msg.reply(ctx, content).await?;
            }
        }

    }

    Ok(())
}

#[command]
#[aliases("action", "do")]
async fn action(ctx: &Context, msg: &Message) -> CommandResult {

    let command_name = msg.content.to_owned().clone().split(" ").collect::<Vec<&str>>().get(1).and_then(|x| Some(x.to_string()));

    println!("Action - {:?}", &command_name);

    match command_name {
        None => {
            msg.reply(ctx, "invalid command").await?;
        },
        Some(command) => {
            let user_lock = {
                let data_read = ctx.data.read().await;
                data_read.get::<StoryListenerContainer>().expect("Expected StoryListenerContainer in TypeMap").clone()
                
            };

            {
                let mut user_map = user_lock.write().await;
                let mut user = user_map.get(&msg.author.id);
                let mut new_user_value = None;
                match user {
                    None => {
                        msg.reply(ctx, "User hasn't started the story yet").await?;

                    },
                    Some(user_prime) => {
                        println!("Inside user_prime block");
                        let mut index = -1;
                        // for i in user_prime.current_story_path.unwrap_or(Arc::new(StoryBlock2::new("temp"))).path.into_iter() {
                            match &user_prime.current_story_path {
                                None => {
                                    msg.reply(ctx, "No active story").await?;
                                },
                                Some(val) => {
                                    for (i, data) in val.path.iter().enumerate() {
                                        if data.1 == command {
                                            index = i as i32;
                                        }
                                        
                                        println!("\n\n{:?}\n\n", data);
                                    }
                                    
                                    println!("Index - {index}");
                                    if index == -1 {
                                        user = None;
                                    } else {
                                        new_user_value = Some(StoryListener::new(&val.path.get(index as usize).unwrap().0));
                                    }
                                } 
                            }
                    }
                }
                // user = new_user_value;

                println!("{:?}", &user);
                println!("{:?}", &new_user_value);
                if new_user_value.is_some() {
                    let val = user_map.insert(msg.author.id.clone(), new_user_value.unwrap());
                    if let Some(st) = &val {
                        if let Some(message) = &st.current_story_path {
                            
                            msg.reply(ctx, message.present()).await?;
                        }
                    }
                }
            }
        }
    }

    

    Ok(())
}

#[command]
#[owners_only]
async fn load(ctx: &Context, msg: &Message) -> CommandResult {

    //TODO: there probably is a better way of doing thins
    let file_path = msg.content.to_owned().clone().split(" ").collect::<Vec<&str>>().get(1).and_then(|x| Some(x.to_string()));
    println!("{:?}", &file_path);
    if let None = file_path {
        msg.reply(ctx, "Error occurred during parsing of the command. (File path not supplied?)").await?;
        return Ok(());
    }
    let file_path = file_path.unwrap();
    let story = map_stories(&file_path);
    let file_name = Path::new(&file_path);
    let file_name = file_name.file_stem().unwrap().to_str().unwrap().to_string();

    match story {
        Ok(story) => {
            let story_lock = {
                let data_read = ctx.data.read().await;
                data_read.get::<StoryContainer2>().expect("Expected StoryContainer2 in TypeMap").clone()
            };
            
            {
                let mut map = story_lock.write().await;
                map.insert(file_name, Arc::new(story));
            }
        },
        Err(err) => {
            msg.reply(ctx, err).await?;
            return Ok(());
        }
    }
    msg.reply(ctx, "Loaded successfully").await?;
    Ok(())
}

#[command]
#[owners_only]
async fn read_loaded(ctx: &Context, msg: &Message) -> CommandResult {

    let story_lock = {
        let data_read = ctx.data.read().await;
        data_read.get::<StoryContainer2>().expect("Expected StoryContainer2 in TypeMap").clone()
    };
    {
        let stories = story_lock.read().await.clone();
        
        println!("{:?}", &stories);
        let message = stories.into_keys()
            .collect::<Vec<String>>()
            .iter()
            .map(|x| x.to_owned() + "\n")
            .collect::<Vec<String>>()
            .concat();

        msg.reply(ctx, message).await?;
    }

    Ok(())
}

#[command]
#[owners_only]
async fn set_story(ctx: &Context, msg: &Message) -> CommandResult {

     //TODO: there probably is a better way of doing thins
    let story_name = msg.content.to_owned().clone().split(" ").collect::<Vec<&str>>().get(1).and_then(|x| Some(x.to_string()));
    if let None = story_name {
        msg.reply(ctx, "Error occurred during parsing of the command. (File path not supplied?)").await?;
        return Ok(());
    }
    let story_name = story_name.unwrap();
    
    let (curr_story_lock, story_lock) = {
        let data_read = ctx.data.read().await;

        (data_read.get::<LoadedStoryContainer>().expect("Expected LoadedStoryContainer in TypeMap").clone(),
         data_read.get::<StoryContainer2>().expect("Expected StoryContainer2 in TypeMap").clone())
    };

    {
        let story_map = story_lock.read().await;
        let story = story_map.get(&story_name);

        match story {
            None => {
                msg.reply(ctx, "There is no such story").await?;
            },
            Some(story) => {
                let mut current_story = curr_story_lock.write().await;
                current_story.replace(story.clone());
            
                msg.reply(ctx, "Loaded the story").await?;
            }
        }
    }

    Ok(())
}