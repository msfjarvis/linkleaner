use crate::{
    dice::roll_die,
    fixer::FixerState,
    message::BotExt,
    utils::{extract_dice_count, parse_bool, AsyncError},
    FIXER_STATE,
};
use std::{env, sync::LazyLock};
use teloxide::{
    prelude::Requester,
    types::{Message, UserId},
    utils::command::BotCommands,
    Bot,
};

pub(crate) type FilterState = String;

const NOT_AUTHORIZED: &str = "You are not authorized for this action";

static BOT_OWNER: LazyLock<UserId> = LazyLock::new(|| {
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
    #[command(description = "toggle Reddit link replacement")]
    Reddit { filter_state: FilterState },
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
    let from = message.from.as_ref().ok_or("No user found")?;
    Ok(from.id == *BOT_OWNER || admins.contains(from))
}

fn update_fixer_state(message: &Message, update_state: impl FnOnce(&mut FixerState) + Copy) {
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

fn get_fixer_state(message: &Message, get_state: impl FnOnce(&FixerState) -> bool) -> &'static str {
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
            bot.reply(&message, &Command::descriptions().to_string())
                .await?;
        }
        Command::Ping => {
            bot.reply(&message, "Pong").await?;
        }
        Command::Instagram { filter_state } => {
            flip_filter_state(
                &bot,
                &message,
                filter_state,
                "Instagram",
                |state| state.instagram,
                FixerState::instagram,
            )
            .await?;
        }
        Command::Medium { filter_state } => {
            flip_filter_state(
                &bot,
                &message,
                filter_state,
                "Medium",
                |state| state.medium,
                FixerState::medium,
            )
            .await?;
        }
        Command::Reddit { filter_state } => {
            flip_filter_state(
                &bot,
                &message,
                filter_state,
                "Reddit",
                |state| state.reddit,
                FixerState::reddit,
            )
            .await?;
        }
        Command::Ttv { names } => {
            let text = format!("https://twitchtheater.tv/{}", names.replace(' ', "/"));
            bot.reply(&message, &text).await?;
        }
        Command::Twitter { filter_state } => {
            flip_filter_state(
                &bot,
                &message,
                filter_state,
                "Twitter",
                |state| state.twitter,
                FixerState::twitter,
            )
            .await?;
        }
        Command::YouTube { filter_state } => {
            flip_filter_state(
                &bot,
                &message,
                filter_state,
                "YouTube",
                |state| state.youtube,
                FixerState::youtube,
            )
            .await?;
        }
        Command::Dice { size } => match extract_dice_count(&size, 6) {
            Ok(size) => {
                let roll = roll_die(size);
                bot.reply(
                    &message,
                    &format!("You roll a <b>D{size}</b> and get a <b>{roll}</b>."),
                )
                .await?;
            }
            Err(error_message) => {
                bot.reply(&message, &error_message).await?;
            }
        },
    };
    Ok(())
}

async fn flip_filter_state(
    bot: &Bot,
    message: &Message,
    filter_state: String,
    fixer_name: &str,
    current_state: impl Fn(&FixerState) -> bool,
    update_state: impl FnOnce(&mut FixerState, bool) + Copy,
) -> Result<(), AsyncError> {
    if filter_state.is_empty() {
        bot.reply(
            message,
            &format!(
                "{fixer_name} link replacement is {}",
                get_fixer_state(message, current_state)
            ),
        )
        .await?;
        return Ok(());
    }
    if check_authorized(bot, message).await? {
        match parse_bool(&filter_state) {
            Ok(filter_state) => {
                update_fixer_state(message, |x| update_state(x, filter_state));
                let state = if filter_state { "enabled" } else { "disabled" };
                bot.reply(
                    message,
                    &format!("{fixer_name} link replacement is now {state}"),
                )
                .await?
            }
            Err(error_message) => bot.reply(message, &error_message).await?,
        };
    } else {
        bot.reply(message, NOT_AUTHORIZED).await?;
    }
    Ok(())
}
