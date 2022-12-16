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
    #[command(description = "enable or disable Instagram link replacement")]
    Ddinstagram { filter_state: FilterState },
    #[command(description = "enable or disable Twitter link replacement")]
    Vxtwitter { filter_state: FilterState },
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
        Command::Ddinstagram { filter_state } => {
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
        Command::Vxtwitter { filter_state } => {
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
    };
    Ok(())
}
