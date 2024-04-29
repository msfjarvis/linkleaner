#![feature(let_chains)]
mod commands;
mod deamp;
mod fixer;
#[cfg(feature = "ddinstagram")]
mod instagram;
mod logging;
mod medium;
mod message;
mod twitter;
mod utils;
mod youtube;

use crate::commands::Command;
use crate::logging::TeloxideLogger;
use dotenvy::dotenv;
use fixer::FixerState;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use teloxide::{
    dispatching::{dialogue::GetChatId, HandlerExt, UpdateFilterExt},
    dptree,
    prelude::Dispatcher,
    types::{ChatId, Message, Update},
    update_listeners::Polling,
    Bot,
};

pub(crate) static FIXER_STATE: Lazy<Mutex<HashMap<ChatId, FixerState>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
const REPLACE_SKIP_TOKEN: &str = "#skip";

async fn run() {
    if let Err(e) = logging::init() {
        eprintln!("{e}");
        return;
    };
    dotenv().ok();

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(commands::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                if let Ok(ref mut map) = FIXER_STATE.try_lock()
                    && let Some(chat_id) = msg.chat_id()
                {
                    let state = map.entry(chat_id).or_insert(FixerState::default());
                    return state.twitter
                        && msg
                            .text()
                            .map(|text| {
                                twitter::MATCH_REGEX.is_match(text)
                                    && !text.contains(REPLACE_SKIP_TOKEN)
                            })
                            .unwrap_or_default();
                }
                false
            })
            .endpoint(twitter::handler),
        );
    #[cfg(feature = "ddinstagram")]
    let handler = handler.branch(
        dptree::filter(|msg: Message| {
            if let Ok(ref mut map) = FIXER_STATE.try_lock()
                && let Some(chat_id) = msg.chat_id()
            {
                let state = map.entry(chat_id).or_insert(FixerState::default());
                return state.instagram
                    && msg
                        .text()
                        .map(|text| {
                            instagram::MATCH_REGEX.is_match(text)
                                && !text.contains(REPLACE_SKIP_TOKEN)
                        })
                        .unwrap_or_default();
            }
            false
        })
        .endpoint(instagram::handler),
    );
    let handler = handler.branch(
        dptree::filter(|msg: Message| {
            if let Ok(ref mut map) = FIXER_STATE.try_lock()
                && let Some(chat_id) = msg.chat_id()
            {
                let state = map.entry(chat_id).or_insert(FixerState::default());
                return state.youtube
                    && msg
                        .text()
                        .map(|text| {
                            youtube::MATCH_REGEX.is_match(text)
                                && !text.contains(REPLACE_SKIP_TOKEN)
                        })
                        .unwrap_or_default();
            }
            false
        })
        .endpoint(youtube::handler),
    );
    let handler = handler.branch(
        dptree::filter(|msg: Message| {
            if let Ok(ref mut map) = FIXER_STATE.try_lock()
                && let Some(chat_id) = msg.chat_id()
            {
                let state = map.entry(chat_id).or_insert(FixerState::default());
                return state.medium
                    && msg
                        .text()
                        .map(|text| {
                            medium::MATCH_REGEX.is_match(text) && !text.contains(REPLACE_SKIP_TOKEN)
                        })
                        .unwrap_or_default();
            }
            false
        })
        .endpoint(medium::handler),
    );

    let handler = handler.branch(dptree::filter(deamp::is_amp).endpoint(deamp::handler));

    let error_handler = Arc::new(TeloxideLogger::default());
    let listener = Polling::builder(bot.clone()).drop_pending_updates().build();
    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(listener, error_handler)
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}
