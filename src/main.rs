#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(let_chains)]
mod commands;
mod deamp;
#[cfg(feature = "ddinstagram")]
mod instagram;
mod logging;
mod message;
mod twitter;
mod utils;
mod youtube;

use crate::commands::Command;
use crate::logging::TeloxideLogger;
use dotenvy::dotenv;
use std::sync::{atomic::Ordering, Arc};
use teloxide::{
    dispatching::{update_listeners::Polling, HandlerExt, UpdateFilterExt},
    dptree,
    prelude::Dispatcher,
    types::{Message, Update},
    Bot,
};

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
                twitter::FILTER_ENABLED.load(Ordering::Relaxed)
                    && msg
                        .text()
                        .map(|text| {
                            twitter::MATCH_REGEX.is_match(text)
                                && !text.contains(REPLACE_SKIP_TOKEN)
                        })
                        .unwrap_or_default()
            })
            .endpoint(twitter::handler),
        );
    #[cfg(feature = "ddinstagram")]
    let handler = handler.branch(
        dptree::filter(|msg: Message| {
            instagram::FILTER_ENABLED.load(Ordering::Relaxed)
                && msg
                    .text()
                    .map(|text| {
                        instagram::MATCH_REGEX.is_match(text) && !text.contains(REPLACE_SKIP_TOKEN)
                    })
                    .unwrap_or_default()
        })
        .endpoint(instagram::handler),
    );
    let handler = handler.branch(
        dptree::filter(|msg: Message| {
            youtube::FILTER_ENABLED.load(Ordering::Relaxed)
                && msg
                    .text()
                    .map(|text| {
                        youtube::MATCH_REGEX.is_match(text) && !text.contains(REPLACE_SKIP_TOKEN)
                    })
                    .unwrap_or_default()
        })
        .endpoint(youtube::handler),
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
