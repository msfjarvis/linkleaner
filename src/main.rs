#![allow(clippy::too_many_lines)]
mod bot_ext;
mod commands;
mod deamp;
mod dice;
mod fixer;
mod instagram;
mod logging;
mod medium;
mod reddit;
mod router_ext;
mod tiktok;
mod twitter;
mod url;
mod youtube;

use crate::{
    commands::Command,
    logging::TeloxideLogger,
    url::{get_urls_from_message, has_matching_urls},
};
use dotenvy::dotenv;
use fixer::FixerState;
use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, LazyLock, Mutex},
};
use teloxide::{
    Bot,
    dispatching::{HandlerExt, UpdateFilterExt, dialogue::GetChatId},
    dptree,
    prelude::Dispatcher,
    types::{ChatId, Message, Update},
    update_listeners::Polling,
};

pub(crate) type AsyncError = Box<dyn Error + Send + Sync + 'static>;

pub(crate) static FIXER_STATE: LazyLock<Mutex<HashMap<ChatId, FixerState>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
const REPLACE_SKIP_TOKEN: &str = "#skip";

async fn run() {
    if let Err(e) = logging::init() {
        eprintln!("{e}");
        return;
    }
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
                if should_match(&msg, &twitter::DOMAINS)
                    && let Ok(ref mut map) = FIXER_STATE.try_lock()
                    && let Some(chat_id) = msg.chat_id()
                {
                    return map.entry(chat_id).or_insert(FixerState::default()).twitter;
                }
                false
            })
            .endpoint(twitter::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                if should_match(&msg, &instagram::DOMAINS)
                    && let Ok(ref mut map) = FIXER_STATE.try_lock()
                    && let Some(chat_id) = msg.chat_id()
                {
                    return map
                        .entry(chat_id)
                        .or_insert(FixerState::default())
                        .instagram;
                }
                false
            })
            .endpoint(instagram::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                if should_match(&msg, &youtube::DOMAINS)
                    && let Ok(ref mut map) = FIXER_STATE.try_lock()
                    && let Some(chat_id) = msg.chat_id()
                {
                    return map.entry(chat_id).or_insert(FixerState::default()).youtube;
                }
                false
            })
            .endpoint(youtube::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                if should_match(&msg, &medium::DOMAINS)
                    && let Ok(ref mut map) = FIXER_STATE.try_lock()
                    && let Some(chat_id) = msg.chat_id()
                {
                    return map.entry(chat_id).or_insert(FixerState::default()).medium;
                }
                false
            })
            .endpoint(medium::handler),
        )
        .branch(
            dptree::filter(|msg| {
                if should_match(&msg, &reddit::DOMAINS)
                    && let Ok(ref mut map) = FIXER_STATE.try_lock()
                    && let Some(chat_id) = msg.chat_id()
                {
                    return map.entry(chat_id).or_insert(FixerState::default()).reddit;
                }
                false
            })
            .endpoint(reddit::handler),
        )
        .branch(
            dptree::filter(|msg| {
                if should_match(&msg, &tiktok::DOMAINS)
                    && let Ok(ref mut map) = FIXER_STATE.try_lock()
                    && let Some(chat_id) = msg.chat_id()
                {
                    return map.entry(chat_id).or_insert(FixerState::default()).tiktok;
                }
                false
            })
            .endpoint(tiktok::handler),
        )
        .branch(
            dptree::filter(|msg| {
                let urls = get_urls_from_message(&msg);
                if urls.is_empty() {
                    false
                } else {
                    urls.iter().any(deamp::is_amp)
                }
            })
            .endpoint(deamp::handler),
        )
        .branch(dptree::filter(dice::is_die_roll).endpoint(dice::handler));

    let error_handler = Arc::new(TeloxideLogger::default());
    let listener = Polling::builder(bot.clone()).drop_pending_updates().build();
    Box::pin(
        Dispatcher::builder(bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch_with_listener(listener, error_handler),
    )
    .await;
}

fn should_match(msg: &Message, domains: &[&str]) -> bool {
    if msg.text().unwrap_or_default().contains(REPLACE_SKIP_TOKEN) {
        return false;
    }
    has_matching_urls(msg, domains)
}

#[tokio::main]
async fn main() {
    Box::pin(run()).await;
}
