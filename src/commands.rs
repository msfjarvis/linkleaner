use crate::utils::parse_bool;
use once_cell::sync::Lazy;
use std::{env, error::Error, marker::Send};
use teloxide::{
    payloads::SendMessageSetters,
    prelude::Requester,
    types::{ChatAction, Message, UserId},
    utils::command::BotCommands,
    Bot,
};

pub(crate) type FilterState = String;
static BOT_OWNER: Lazy<UserId> = Lazy::new(|| {
    let value = env::var("BOT_OWNER_ID").expect("BOT_OWNER_ID must be defined");
    let id = value
        .parse::<u64>()
        .expect("BOT_OWNER_ID must be a valid integer");
    UserId(id)
});

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub(crate) enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Pong?")]
    Ping,
    #[cfg(feature = "ddinstagram")]
    #[command(description = "toggle Instagram link replacement")]
    Instagram { filter_state: FilterState },
    #[command(description = "toggle Medium link replacement")]
    Medium { filter_state: FilterState },
    #[command(description = "generate a twitchtheater link for the given streamers")]
    Ttv { names: String },
    #[command(description = "toggle Twitter link replacement")]
    Twitter { filter_state: FilterState },
    #[command(description = "toggle YouTube link replacement")]
    YouTube { filter_state: FilterState },
}

pub(crate) async fn handler(
    bot: Bot,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_chat_action(message.chat.id, ChatAction::Typing)
                .await?;
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Ping => {
            bot.send_chat_action(message.chat.id, ChatAction::Typing)
                .await?;
            bot.send_message(message.chat.id, "Pong")
                .await?;
        },
        #[cfg(feature = "ddinstagram")]
        Command::Instagram { filter_state } => {
            if let Some(from) = message.from() && from.id != *BOT_OWNER {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            } else {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        crate::instagram::set_filter_state(bot, message, filter_state).await?;
                    }
                    Err(error_message) => {
                        bot.send_chat_action(message.chat.id, ChatAction::Typing)
                            .await?;
                        bot.send_message(message.chat.id, error_message)
                            .reply_to_message_id(message.id)
                            .await?;
                    }
                }
            }
        }
        Command::Medium { filter_state } => {
            if let Some(from) = message.from() && from.id != *BOT_OWNER {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            } else {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        crate::medium::set_filter_state(bot, message, filter_state).await?;
                    }
                    Err(error_message) => {
                        bot.send_chat_action(message.chat.id, ChatAction::Typing)
                            .await?;
                        bot.send_message(message.chat.id, error_message)
                            .reply_to_message_id(message.id)
                            .await?;
                    }
                }
            }
        }
        Command::Ttv { names } => {
            let text = format!("https://twitchtheater.tv/{}", names.replace(' ', "/"));
            bot.send_message(message.chat.id, text)
                .reply_to_message_id(message.id)
                .await?;
        }
        Command::Twitter { filter_state } => {
            if let Some(from) = message.from() && from.id != *BOT_OWNER {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            } else {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        crate::twitter::set_filter_state(bot, message, filter_state).await?;
                    }
                    Err(error_message) => {
                        bot.send_chat_action(message.chat.id, ChatAction::Typing)
                            .await?;
                        bot.send_message(message.chat.id, error_message)
                            .reply_to_message_id(message.id)
                            .await?;
                    }
                }
            }
        }
        Command::YouTube { filter_state } => {
            if let Some(from) = message.from() && from.id != *BOT_OWNER {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            } else {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        crate::youtube::set_filter_state(bot, message, filter_state).await?;
                    }
                    Err(error_message) => {
                        bot.send_chat_action(message.chat.id, ChatAction::Typing)
                            .await?;
                        bot.send_message(message.chat.id, error_message)
                            .reply_to_message_id(message.id)
                            .await?;
                    }
                }
            }
        }
    };
    Ok(())
}
