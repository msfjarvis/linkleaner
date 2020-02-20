mod utils;

use teloxide::types::{InputFile, ParseMode};
use teloxide::{prelude::*, utils::command::BotCommand};

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::env;
use walkdir::WalkDir;

use utils::escape_markdown_str;

lazy_static! {
    static ref FILES: Vec<String> = index_pictures();
    static ref FILE_COUNT: usize = FILES.len();
    static ref BASE_URL: String = env::var("BASE_URL").unwrap();
    static ref BASE_DIR: String = env::var("BASE_DIR").unwrap();
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Return a random picture")]
    Random,
}

async fn get_random_file() -> String {
    FILES
        .get(thread_rng().gen_range(0, *FILE_COUNT))
        .unwrap()
        .clone()
}

async fn answer(cx: DispatcherHandlerCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Random => {
            let file = get_random_file().await;
            let link = format!("{}/{}", *BASE_URL, file);
            cx.answer_photo(InputFile::url(&link))
                .caption(format!(
                    "[{}]({})",
                    &escape_markdown_str(file).replace(".jpg", ""),
                    link
                ))
                .parse_mode(ParseMode::MarkdownV2)
                .reply_to_message_id(cx.update.id)
                .send()
                .await?
        }
    };

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    // Only iterate through commands in a proper format:
    rx.commands::<Command>()
        // Execute all incoming commands concurrently:
        .for_each_concurrent(None, |(cx, command, _)| async move {
            answer(cx, command).await.log_on_error().await;
        })
        .await;
}

fn index_pictures() -> Vec<String> {
    let mut images: Vec<String> = Vec::new();
    for entry in WalkDir::new(&**BASE_DIR).into_iter().filter_map(|e| e.ok()) {
        images.push(String::from(
            entry
                .path()
                .strip_prefix(&**BASE_DIR)
                .unwrap()
                .to_str()
                .unwrap(),
        ))
    }
    images
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot!");

    log::debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(handle_commands)
        .dispatch()
        .await;
}
