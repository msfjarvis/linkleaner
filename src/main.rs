mod commands;
mod utils;

use dotenv::dotenv;

use teloxide::{
    adaptors::auto_send::AutoRequest,
    payloads::SendPhoto,
    requests::MultipartRequest,
    types::{ChatAction, InputFile, ParseMode},
};
use teloxide::{prelude::*, utils::command::BotCommand};

use lazy_static::lazy_static;
use std::{env, error::Error, path::PathBuf};

use crate::commands::Command;
use crate::utils::{
    file_name_to_label, get_random_file, get_search_results, index_pictures, join_results_to_string,
};

lazy_static! {
    static ref FILES: Vec<String> = index_pictures(&**BASE_DIR);
    static ref BASE_URL: String = env::var("BASE_URL").expect("BASE_URL must be defined");
    static ref BASE_DIR: String = env::var("BASE_DIR").expect("BASE_DIR must be defined");
    static ref BOT_NAME: String = env::var("BOT_NAME").unwrap_or_default();
}

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;

fn search(search_term: &str) -> Vec<String> {
    get_search_results((*FILES).clone(), search_term)
}

fn send_captioned_picture(
    cx: Cx,
    link: String,
    path: &str,
) -> AutoRequest<MultipartRequest<SendPhoto>> {
    let file = InputFile::File(PathBuf::from(format!("{}/{}", *BASE_DIR, path)));
    cx.answer_photo(file)
        .caption(format!("[{}]({})", &file_name_to_label(path), link))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(cx.update.id)
}

async fn answer(cx: Cx, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            cx.requester
                .send_chat_action(cx.update.chat.id, ChatAction::Typing)
                .await?;
            cx.answer(Command::descriptions()).await?;
        }
        Command::Pic { search_term } => {
            if search_term.is_empty() {
                cx.requester
                    .send_chat_action(cx.update.chat.id, ChatAction::Typing)
                    .await?;
                cx.answer("No search query passed")
                    .reply_to_message_id(cx.update.id)
                    .await?;
            } else {
                let results = search(&search_term.replace(" ", "_"));
                if results.is_empty() {
                    cx.requester
                        .send_chat_action(cx.update.chat.id, ChatAction::Typing)
                        .await?;
                    cx.answer(format!("No picture found for '{}'", search_term))
                        .reply_to_message_id(cx.update.id)
                        .await?;
                } else {
                    let file = get_random_file(results);
                    let link = format!("{}/{}", *BASE_URL, file);
                    cx.requester
                        .send_chat_action(cx.update.chat.id, ChatAction::UploadPhoto)
                        .await?;
                    send_captioned_picture(cx, link, &file).await?;
                }
            }
        }
        Command::Random => {
            let file = get_random_file((*FILES).clone());
            let link = format!("{}/{}", *BASE_URL, file);
            cx.requester
                .send_chat_action(cx.update.chat.id, ChatAction::UploadPhoto)
                .await?;
            send_captioned_picture(cx, link, &file).await?;
        }
        Command::Search { search_term } => {
            cx.requester
                .send_chat_action(cx.update.chat.id, ChatAction::Typing)
                .await?;
            let res = search(&search_term);
            if res.is_empty() {
                cx.answer(format!("No results found for '{}'", search_term))
                    .reply_to_message_id(cx.update.id)
                    .await?;
            } else {
                cx.answer(join_results_to_string(search_term, res, &**BASE_URL))
                    .parse_mode(ParseMode::MarkdownV2)
                    .disable_web_page_preview(true)
                    .reply_to_message_id(cx.update.id)
                    .await?;
            }
        }
    };
    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv().ok();
    teloxide::enable_logging!();

    log::debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, BOT_NAME.as_str(), answer).await;
}
