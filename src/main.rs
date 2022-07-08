mod commands;
mod logging;
mod utils;
mod vxtwitter;
mod walls;

use crate::walls::{BASE_DIR, FILES};
use teloxide::{
    dispatching::{HandlerExt, UpdateFilterExt},
    dptree,
    prelude::{Dispatcher, RequesterExt},
    types::Update,
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
    #[cfg(feature = "console")]
    console_subscriber::init();
    dotenv().ok();

    if FILES.is_empty() {
        error!("Failed to index files from {}", *BASE_DIR);
        return;
    }
    debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env().auto_send();

    let handler = dptree::entry()
        .branch(
            Update::filter_message()
                .filter_command::<Command>()
                .endpoint(walls::handler),
        )
        .branch(Update::filter_message().endpoint(vxtwitter::handler));

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}
