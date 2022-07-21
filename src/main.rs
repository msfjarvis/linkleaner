mod commands;
mod ddinstagram;
mod logging;
mod utils;
mod vxtwitter;
mod walls;

use crate::walls::{BASE_DIR, FILES};
use teloxide::{
    dispatching::{HandlerExt, UpdateFilterExt},
    dptree,
    prelude::{Dispatcher, RequesterExt},
    types::{Message, Update},
    Bot,
};
use tracing::error;

use dotenv::dotenv;
use tracing::debug;

use crate::commands::Command;

async fn run() {
    if let Err(e) = logging::init() {
        error!(?e);
        return;
    };
    dotenv().ok();

    if FILES.is_empty() {
        error!("Failed to index files from {}", *BASE_DIR);
        return;
    }
    debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env().auto_send();

    let handler = Update::filter_message()
        .branch(
            dptree::entry()
                .filter_command::<Command>()
                .endpoint(walls::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                msg.text()
                    .map(|text| vxtwitter::MATCH_REGEX.is_match(text))
                    .unwrap_or_default()
            })
            .endpoint(vxtwitter::handler),
        )
        .branch(
            dptree::filter(|msg: Message| {
                msg.text()
                    .map(|text| ddinstagram::MATCH_REGEX.is_match(text))
                    .unwrap_or_default()
            })
            .endpoint(ddinstagram::handler),
        );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}
