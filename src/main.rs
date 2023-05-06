mod commands;
mod story;
mod utilities;

use std::collections::{HashMap, HashSet};
use std::env;
use std::path::Path;
use std::sync::Arc;

use serenity::async_trait;
use serenity::framework::standard::macros::{group, help};
use serenity::framework::standard::{
    help_commands, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::framework::*;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::*;
use std::fs;
use story::story_builder::map_stories_p;
use story::story_structs::StoryContainer;
use tracing::{error, info};
use update_informer::{registry, Check};

use crate::commands::general::*;
use crate::commands::math::*;
use crate::commands::owner::*;
use crate::story::story::*;

pub struct ShardManagerContainer;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}

#[group]
#[commands(info, action, multiply, quit)]
struct General;

#[group]
#[commands(start_story, action, load, read_loaded, set_story, clear_story)]
#[prefixes("story", "s")]
#[description = "Commands related to the stories"]
#[default_command(action)]
struct Story;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    run_informer().await;
    let startup_story_files = check_directory().await;
    run_bot(startup_story_files).await;
}

async fn run_bot(startup_story_files: Option<Vec<String>>) {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&STORY_GROUP);

    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    setup_data(&client, startup_story_files).await;

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    info!("Bot is starting...");
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}

async fn check_directory() -> Option<Vec<String>> {
    let current_directory = env::current_dir().ok()?;

    let mut stories_directory = current_directory.clone();
    stories_directory.push("stories");

    let f1: Option<Vec<String>> = scan_directory(&current_directory);
    let f2: Option<Vec<String>> = scan_directory(&stories_directory);

    let files = combine_file_paths(f1, f2);
    files
}

fn combine_file_paths(f1: Option<Vec<String>>, f2: Option<Vec<String>>) -> Option<Vec<String>> {
    if f1.is_none() && f2.is_some() {
        f2
    } else if f2.is_none() && f1.is_some() {
        f1
    } else {
        f1.and_then(|r1| {
            f2.map(|r2| {
                r1.into_iter()
                    .chain(r2.into_iter())
                    .collect::<Vec<String>>()
            })
        })
    }
}

fn scan_directory(path: &dyn AsRef<Path>) -> Option<Vec<String>> {
    let mut files = Vec::new();
    let directory = fs::read_dir(path).ok()?;

    for dir_entry in directory {
        if let Ok(file) = dir_entry {
            let file_path = file.path();

            if let Some(ext) = file_path.extension() {
                if ext.eq_ignore_ascii_case("story") {
                    if let Some(file_path_str) = file_path.to_str() {
                        files.push(file_path_str.to_owned());
                    }
                }
            }
        }
    }

    Some(files)
}

async fn run_informer() {
    let informer = update_informer::new(
        registry::GitHub,
        "https://github.com/ErnestasSku/Mnemosyne",
        "0.1.0",
    );

    if let Some(version) = informer.check_version().ok().flatten() {
        println!("New version is available: {}. Go to https://github.com/ErnestasSku/Mnemosyne to update", version);
    }
}

async fn setup_data(client: &Client, startup_story_files: Option<Vec<String>>) {
    let mut data = client.data.write().await;
    data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    data.insert::<StoryListenerContainer>(Arc::new(RwLock::new(HashMap::default())));
    data.insert::<LoadedStoryContainer>(Arc::new(RwLock::new(None)));

    info!("Setup info");
    if let Some(files) = startup_story_files {
        let stories = parse_stories(&files);
        info!("Inserting {} stories", &stories.len());
        data.insert::<StoryContainer>(Arc::new(RwLock::new(stories)));
    } else {
        data.insert::<StoryContainer>(Arc::new(RwLock::new(HashMap::default())));
    }
}

fn parse_stories(files: &[String]) -> HashMap<String, Arc<story::story_structs::StoryBlock>> {
    let mut counter = 0;
    files
        .iter()
        .filter_map(|f| {
            let file_path = Path::new(f).to_owned();
            let name = file_path.file_stem()?;
            let story = map_stories_p(f).ok()?;
            let name_str = name.to_str().map(String::from).unwrap_or_else(|| {
                format!("default-{}", {
                    counter += 1;
                    counter - 1
                })
            });
            Some((name_str, story))
        })
        .collect()
}

#[help]
#[command_not_found_text = "Could not find: `{}`."]
#[max_levenshtein_distance(3)]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
