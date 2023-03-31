use std::{collections::HashMap, sync::Arc};

use serenity::{
    client::bridge::gateway::ShardManager,
    model::prelude::UserId,
    prelude::{Mutex, RwLock, TypeMapKey},
};

use crate::{
    story::{
        story::{LoadedStoryContainer, StoryListener, StoryListenerContainer},
        story_structs::{StoryBlock, StoryContainer},
    },
    ShardManagerContainer,
};

pub type ArcMutex<T> = Arc<Mutex<T>>;
pub type ArcLock<T> = Arc<RwLock<T>>;
pub type UserListenerMap = HashMap<UserId, StoryListener>;
pub type LoadedStory = Option<(Arc<StoryBlock>, String)>;
pub type StoryMap = HashMap<String, Arc<StoryBlock>>;

impl TypeMapKey for ShardManagerContainer {
    type Value = ArcMutex<ShardManager>;
}

impl TypeMapKey for StoryListenerContainer {
    type Value = ArcLock<UserListenerMap>;
}

impl TypeMapKey for LoadedStoryContainer {
    type Value = ArcLock<LoadedStory>;
}

impl TypeMapKey for StoryContainer {
    type Value = ArcLock<StoryMap>;
}
