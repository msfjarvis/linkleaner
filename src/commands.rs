use crate::{
    dice::roll_die,
    fixer::FixerState,
    message::BotExt,
    utils::{extract_dice_count, parse_bool, AsyncError},
    FIXER_STATE,
};
use once_cell::sync::Lazy;
use std::env;
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
    #[command(description = "toggle Instagram link replacement")]
    Instagram { filter_state: FilterState },
    #[command(description = "toggle Medium link replacement")]
    Medium { filter_state: FilterState },
    #[command(description = "display this text.")]
    Start,
    #[command(description = "generate a twitchtheater link for the given streamers")]
    Ttv { names: String },
    #[command(description = "toggle Twitter link replacement")]
    Twitter { filter_state: FilterState },
    #[command(description = "toggle YouTube link replacement")]
    YouTube { filter_state: FilterState },
    #[command(
        description = "let the bot decide your fate; pass a number argument to roll dice with a custom number of sides",
        parse_with = "split",
        aliases = ["roll", "d"]
    )]
    Dice { size: String },
}

async fn check_authorized(bot: &Bot, message: &Message) -> Result<bool, AsyncError> {
    if message.chat.is_private() {
        return Ok(true);
    }
    let admins = bot.get_chat_administrators(message.chat.id).await?;
    let admins = admins.iter().map(|c| c.user.clone()).collect::<Vec<_>>();
    let from = message.from().ok_or("No user found")?;
    Ok(from.id == *BOT_OWNER || admins.contains(from))
}

fn update_fixer_state<F>(message: &Message, update_state: F)
where
    F: FnOnce(&mut FixerState) + Copy,
{
    if let Ok(ref mut map) = FIXER_STATE.try_lock() {
        map.entry(message.chat.id)
            .and_modify(update_state)
            .or_insert_with(|| {
                let mut state = FixerState::default();
                update_state(&mut state);
                state
            });
    }
}

fn get_fixer_state<F>(message: &Message, get_state: F) -> &str
where
    F: FnOnce(&FixerState) -> bool + Copy,
{
    if let Ok(ref mut map) = FIXER_STATE.try_lock() {
        let state = map.entry(message.chat.id).or_insert(FixerState::default());
        if get_state(state) {
            return "enabled";
        }
    }
    "disabled"
}

#[allow(clippy::too_many_lines)] // Problem for another day
pub(crate) async fn handler(
    bot: Bot,
    message: Message,
    command: Command,
) -> Result<(), AsyncError> {
    match command {
        Command::Help | Command::Start => {
            bot.try_reply(&message, &Command::descriptions().to_string())
                .await?;
        }
        Command::Ping => {
            bot.try_reply(&message, "Pong").await?;
        }
        Command::Instagram { filter_state } => {
            if check_authorized(&bot, &message).await? {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        update_fixer_state(&message, |x| x.instagram(filter_state));
                        let state = if filter_state { "enabled" } else { "disabled" };
                        bot.try_reply(
                            &message,
                            &format!("Instagram link replacement is now {state}"),
                        )
                        .await?;
                    }
                    Err(error_message) => {
                        if filter_state.is_empty() {
                            bot.try_reply(
                                &message,
                                &format!(
                                    "Instagram link replacement is {}",
                                    get_fixer_state(&message, |x| x.instagram)
                                ),
                            )
                            .await?;
                        } else {
                            bot.try_reply(&message, &error_message).await?;
                        }
                    }
                }
            } else {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            }
        }
        Command::Medium { filter_state } => {
            if check_authorized(&bot, &message).await? {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        update_fixer_state(&message, |x| x.medium(filter_state));
                        let state = if filter_state { "enabled" } else { "disabled" };
                        bot.try_reply(&message, &format!("Medium link replacement is now {state}"))
                            .await?;
                    }
                    Err(error_message) => {
                        if filter_state.is_empty() {
                            bot.try_reply(
                                &message,
                                &format!(
                                    "Medium link replacement is {}",
                                    get_fixer_state(&message, |x| x.medium)
                                ),
                            )
                            .await?;
                        } else {
                            bot.try_reply(&message, &error_message).await?;
                        }
                    }
                }
            } else {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            }
        }
        Command::Ttv { names } => {
            let text = format!("https://twitchtheater.tv/{}", names.replace(' ', "/"));
            bot.send_message(message.chat.id, text)
                .reply_to_message_id(message.id)
                .await?;
        }
        Command::Twitter { filter_state } => {
            if check_authorized(&bot, &message).await? {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        update_fixer_state(&message, |x| x.twitter(filter_state));
                        let state = if filter_state { "enabled" } else { "disabled" };
                        bot.try_reply(
                            &message,
                            &format!("Twitter link replacement is now {state}"),
                        )
                        .await?;
                    }
                    Err(error_message) => {
                        if filter_state.is_empty() {
                            bot.try_reply(
                                &message,
                                &format!(
                                    "Twitter link replacement is {}",
                                    get_fixer_state(&message, |x| x.twitter)
                                ),
                            )
                            .await?;
                        } else {
                            bot.try_reply(&message, &error_message).await?;
                        }
                    }
                }
            } else {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            }
        }
        Command::YouTube { filter_state } => {
            if check_authorized(&bot, &message).await? {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        update_fixer_state(&message, |x| x.youtube(filter_state));
                        let state = if filter_state { "enabled" } else { "disabled" };
                        bot.try_reply(
                            &message,
                            &format!("YouTube link replacement is now {state}"),
                        )
                        .await?;
                    }
                    Err(error_message) => {
                        if filter_state.is_empty() {
                            bot.try_reply(
                                &message,
                                &format!(
                                    "YouTube link replacement is {}",
                                    get_fixer_state(&message, |x| x.youtube)
                                ),
                            )
                            .await?;
                        } else {
                            bot.try_reply(&message, &error_message).await?;
                        }
                    }
                }
            } else {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            }
        }
        Command::Dice { size } => match extract_dice_count(&size, 6) {
            Ok(size) => {
                let roll = roll_die(size);
                bot.try_reply(
                    &message,
                    &format!("You roll a <b>D{size}</b> and get a <b>{roll}</b>."),
                )
                .await?;
            }
            Err(error_message) => {
                bot.try_reply(&message, &error_message).await?;
            }
        },
    };
    Ok(())
}
