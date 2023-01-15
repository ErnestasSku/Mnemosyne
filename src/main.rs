mod commands;
mod story;

use std::collections::{HashSet, HashMap};
use std::env;
use std::sync::Arc;

use serenity::async_trait;
use serenity::client::bridge::gateway::ShardManager;
use serenity::framework::standard::{Args, CommandResult, help_commands, HelpOptions, CommandGroup};
use serenity::framework::standard::macros::{group, help};
use serenity::framework::*;
use serenity::http::Http;
use serenity::model::event::ResumedEvent;
use serenity::model::gateway::Ready;
use serenity::model::prelude::{Message, UserId};
use serenity::prelude::*;
use story::story2::StoryContainer2;
use tracing::{error, info};
use update_informer::{registry, Check};
use std::time::Duration;

use crate::commands::owner::*;
use crate::commands::math::*;
use crate::story::story::*;


const UPDATE_CHECK_PERIOD: Duration = Duration::from_secs(60 * 60 * 24);

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

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
#[commands(action, multiply, quit)]
struct General;

#[group]
#[commands(start_story, action, load, read_loaded, set_story)]
#[prefixes("story", "s")]
#[description = "Commands related to the stories"]
#[default_command(action)]
struct Story;

#[tokio::main]
async fn main() {

    let informer = update_informer::new(registry::GitHub, "https://github.com/ErnestasSku/OldManRs", "0.1.0").timeout(UPDATE_CHECK_PERIOD);
    if let Some(version) = informer.check_version().ok().flatten()  {
        println!("New version is available: {}", version);
    }

    dotenv::dotenv().expect("Failed to load .env file");

    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            println!("{:?}", owners);
            (owners, info.id)
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework =
        StandardFramework::new().configure(|c| c.owners(owners)
            .prefix("~"))
            .help(&HELP)
            .group(&GENERAL_GROUP)
            .group(&STORY_GROUP);

    let intents = GatewayIntents::all();
    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    //Data inserts
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        data.insert::<StoryContainer2>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<StoryListenerContainer>(Arc::new(RwLock::new(HashMap::default())));
        data.insert::<LoadedStoryContainer>(Arc::new(RwLock::new(None)));
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
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

