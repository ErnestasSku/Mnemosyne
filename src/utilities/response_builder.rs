use serenity::{
    framework::standard::CommandResult,
    model::prelude::{Message, ReactionType},
    prelude::Context,
};

// NOTE: Consider modifying this to make it into Type
// builder, and/or non-consuming builder.
pub struct MnemosyneResponseBuilder<'a> {
    ctx: &'a Context,
    msg: &'a Message,
    react: Option<bool>,
    content: Option<String>,
    sendable: bool,
    reaction_emoji: Option<char>,
}

pub struct MnemosyneResponse<'a> {
    pub ctx: &'a Context,
    pub msg: &'a Message,
    pub react: bool,
    pub content: String,
    pub reaction_emoji: char,
    sendable: bool,
}

impl<'a> MnemosyneResponse<'a> {
    pub async fn respond(self) -> CommandResult {
        if !self.sendable {
            return Ok(());
        }

        if self.react {
            self.msg.react(self.ctx, self.reaction_emoji).await?;
        } else {
            // TODO: Add reply and send without reply
            self.msg.reply(self.ctx, self.content).await?;
        }

        Ok(())
    }
}

impl<'a> MnemosyneResponseBuilder<'a> {
    pub fn new(ctx: &'a Context, msg: &'a Message) -> Self {
        MnemosyneResponseBuilder {
            ctx: ctx,
            msg: msg,
            react: None,
            content: None,
            sendable: false,
            reaction_emoji: None,
        }
    }

    pub fn set_react_mode(mut self, react: bool) -> Self {
        self.react = Some(react);
        self
    }

    pub fn set_content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self.sendable = true;
        self
    }

    pub fn set_reaction_emoji(mut self, emoji: char) -> Self {
        self.reaction_emoji = Some(emoji);
        self.sendable = true;
        self
    }

    pub fn build(self) -> MnemosyneResponse<'a> {
        MnemosyneResponse {
            ctx: self.ctx,
            msg: self.msg,
            react: self.react.unwrap_or(false),
            content: self.content.unwrap_or(String::from("")),
            sendable: self.sendable,
            reaction_emoji: self.reaction_emoji.unwrap_or('🐛'),
        }
    }
}
