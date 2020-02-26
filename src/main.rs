mod utils;

use dotenv::dotenv;

use teloxide::types::{InputFile, ParseMode};
use teloxide::{prelude::*, utils::command::BotCommand};

use lazy_static::lazy_static;
use rand::{thread_rng, Rng};
use std::env;
use walkdir::WalkDir;

use crate::utils::{file_name_to_label, join_results_to_string, tokenized_search};
use teloxide::requests::{SendChatActionKind, SendPhoto};

lazy_static! {
    static ref FILES: Vec<String> = index_pictures();
    static ref FILE_COUNT: usize = FILES.len();
    static ref BASE_URL: String = env::var("BASE_URL").expect("BASE_URL must be defined");
    static ref BASE_DIR: String = env::var("BASE_DIR").expect("BASE_DIR must be defined");
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "return a picture matching a given query")]
    Pic,
    #[command(description = "return a random picture")]
    Random,
    #[command(description = "search picture based on given string")]
    Search,
}

fn get_random_file(files: Vec<String>) -> String {
    files
        .get(thread_rng().gen_range(0, files.len()))
        .unwrap()
        .to_string()
}

fn get_search_results(search_term: &str) -> Vec<String> {
    FILES
        .clone()
        .into_iter()
        .filter(|x| tokenized_search(x.to_string(), &search_term.to_lowercase()))
        .collect()
}

fn send_captioned_picture(
    cx: DispatcherHandlerCx<Message>,
    link: String,
    file: String,
) -> SendPhoto {
    cx.answer_photo(InputFile::url(&link))
        .caption(format!("[{}]({})", &file_name_to_label(file), link))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(cx.update.id)
}

async fn answer(
    cx: DispatcherHandlerCx<Message>,
    command: Command,
    args: &[String],
) -> ResponseResult<Option<Message>> {
    match command {
        Command::Help => {
            cx.bot
                .send_chat_action(cx.update.chat.id, SendChatActionKind::Typing)
                .send()
                .await?;
            cx.answer(Command::descriptions())
                .reply_to_message_id(cx.update.id)
                .send()
                .await?
        }
        Command::Pic => {
            if args.is_empty() {
                cx.bot
                    .send_chat_action(cx.update.chat.id, SendChatActionKind::Typing)
                    .send()
                    .await?;
                cx.answer("No search query passed")
                    .reply_to_message_id(cx.update.id)
                    .send()
                    .await?
            } else {
                let results = get_search_results(&args.join("_"));
                if results.is_empty() {
                    cx.bot
                        .send_chat_action(cx.update.chat.id, SendChatActionKind::Typing)
                        .send()
                        .await?;
                    cx.answer(format!("No picture found for '{}'", &args.join(" ")))
                        .reply_to_message_id(cx.update.id)
                        .send()
                        .await?
                } else {
                    let file = get_random_file(results);
                    let link = format!("{}/{}", *BASE_URL, file);
                    cx.bot
                        .send_chat_action(cx.update.chat.id, SendChatActionKind::UploadPhoto)
                        .send()
                        .await?;
                    send_captioned_picture(cx, link, file).send().await?
                }
            }
        }
        Command::Random => {
            let file = get_random_file((*FILES).clone());
            let link = format!("{}/{}", *BASE_URL, file);
            cx.bot
                .send_chat_action(cx.update.chat.id, SendChatActionKind::UploadPhoto)
                .send()
                .await?;
            send_captioned_picture(cx, link, file).send().await?
        }
        Command::Search => {
            let search_term = args.join("_");
            let res = get_search_results(&search_term);
            cx.bot
                .send_chat_action(cx.update.chat.id, SendChatActionKind::Typing)
                .send()
                .await?;
            if res.is_empty() {
                cx.answer(format!("No results found for '{}'", &args.join(" ")))
                    .reply_to_message_id(cx.update.id)
                    .send()
                    .await?
            } else {
                cx.answer(join_results_to_string(search_term, res, &**BASE_URL))
                    .parse_mode(ParseMode::MarkdownV2)
                    .disable_web_page_preview(true)
                    .reply_to_message_id(cx.update.id)
                    .send()
                    .await?
            }
        }
    };

    Ok(None)
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    // Only iterate through commands in a proper format:
    rx.commands::<Command>()
        // Execute all incoming commands concurrently:
        .for_each_concurrent(None, |(cx, command, args)| async move {
            return match answer(cx, command, &args).await {
                Ok(Some(msg)) => {
                    if let Some(photo) = msg.photo() {
                        println!("photo: {}", photo.get(0).unwrap().clone().file_unique_id);
                    }
                }
                Ok(None) => {}
                Err(e) => log::error!("{}", e),
            };
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
    dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot!");

    log::debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(handle_commands)
        .dispatch()
        .await;
}
