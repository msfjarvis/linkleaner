mod amputator;
mod commands;
mod ddinstagram;
mod logging;
mod utils;
mod vxtwitter;
mod walls;

use crate::commands::Command;
use crate::logging::TeloxideLogger;
use crate::walls::{BASE_DIR, FILES};
use dotenvy::dotenv;
use std::sync::Arc;
use teloxide::{
    dispatching::{update_listeners::Polling, HandlerExt, UpdateFilterExt},
    dptree,
    prelude::Dispatcher,
    types::{Message, Update},
    Bot,
};
use tracing::debug;
use tracing::error;

const REPLACE_SKIP_TOKEN: &str = "#skip";

async fn run() {
    if let Err(e) = logging::init() {
        eprintln!("{}", e);
        return;
    };
    dotenv().ok();

    if FILES.is_empty() {
        error!("Failed to index files from {}", *BASE_DIR);
        return;
    }
    debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(walls::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                msg.text()
                    .map(|text| {
                        vxtwitter::MATCH_REGEX.is_match(text) && !text.contains(REPLACE_SKIP_TOKEN)
                    })
                    .unwrap_or_default()
            })
            .endpoint(vxtwitter::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                msg.text()
                    .map(|text| {
                        ddinstagram::MATCH_REGEX.is_match(text)
                            && !text.contains(REPLACE_SKIP_TOKEN)
                    })
                    .unwrap_or_default()
            })
            .endpoint(ddinstagram::handler),
        )
        .branch(dptree::filter(amputator::is_amp).endpoint(amputator::handler));

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
