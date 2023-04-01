use serenity::prelude::TypeMap;
use tokio::sync::RwLockReadGuard;

use crate::story::{
    story::{LoadedStoryContainer, StoryListenerContainer},
    story_structs::StoryContainer,
};

use super::type_map::{ArcLock, LoadedStory, StoryMap, UserListenerMap};

type OptArcLock<T> = Option<ArcLock<T>>;

pub struct DataAccessBuilder<'a> {
    data_read: &'a RwLockReadGuard<'a, TypeMap>,
    user_lock: OptArcLock<UserListenerMap>,
    story_lock: OptArcLock<StoryMap>,
    loaded_story_lock: OptArcLock<LoadedStory>,
}

pub struct DataAccess {
    pub user_lock: OptArcLock<UserListenerMap>,
    pub story_lock: OptArcLock<StoryMap>,
    pub loaded_story_lock: OptArcLock<LoadedStory>,
}

impl<'a> DataAccessBuilder<'a> {
    pub fn new(data_read: &'a RwLockReadGuard<'a, TypeMap>) -> Self {
        DataAccessBuilder {
            data_read,
            user_lock: None,
            story_lock: None,
            loaded_story_lock: None,
        }
    }

    pub fn get_user_lock(mut self) -> Self {
        self.user_lock = self
            .data_read
            .get::<StoryListenerContainer>()
            .map(Clone::clone);
        self
    }

    pub fn get_loaded_lock(mut self) -> Self {
        self.loaded_story_lock = self
            .data_read
            .get::<LoadedStoryContainer>()
            .map(Clone::clone);
        self
    }

    pub fn get_story_lock(mut self) -> Self {
        self.story_lock = self.data_read.get::<StoryContainer>().map(Clone::clone);
        self
    }

    pub fn build(self) -> DataAccess {
        DataAccess {
            user_lock: self.user_lock,
            story_lock: self.story_lock,
            loaded_story_lock: self.loaded_story_lock,
        }
    }
}
