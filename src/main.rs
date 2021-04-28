mod commands;
mod utils;

use dotenv::dotenv;

use teloxide::{
    adaptors::auto_send::AutoRequest,
    payloads::{SendDocument, SendPhoto},
    requests::MultipartRequest,
    types::{ChatAction, InputFile, ParseMode},
};
use teloxide::{prelude::*, utils::command::BotCommand};

use lazy_static::lazy_static;
use std::{env, error::Error, path::PathBuf};

use crate::commands::Command;
use crate::utils::{
    file_name_to_label, get_file_hash, get_random_file, get_search_results, index_pictures,
    join_results_to_string,
};

lazy_static! {
    static ref FILES: Vec<String> = index_pictures(&**BASE_DIR);
    static ref BASE_URL: String = env::var("BASE_URL").expect("BASE_URL must be defined");
    static ref BASE_DIR: String = env::var("BASE_DIR").expect("BASE_DIR must be defined");
    static ref BOT_NAME: String = env::var("BOT_NAME").unwrap_or_default();
    // TODO: Remove this once we've figured out why exactly do specific pictures
    // pass the document check yet fail on Telegram.
    static ref FORCE_DOCUMENT: String = env::var("FORCED_DOCUMENT_FILES").unwrap_or_default();
    static ref TREE: sled::Db = sled::open("file_id_cache").unwrap();
}

/// Telegram mandates a photo can not be larger than 10 megabytes
const MAX_FILE_SIZE: u64 = 10485760;

/// Telegram mandates a photo can not be longer than 10000 pixels across any dimension
const MAX_DIMEN: usize = 10000;

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;

fn search(search_term: &str) -> Vec<String> {
    get_search_results((*FILES).clone(), search_term)
}

/// Given a file name, get its path on disk
fn get_file_path(file_name: &str) -> String {
    format!("{}/{}", *BASE_DIR, file_name)
}

/// Given a file name, get its URL
fn get_file_url(file_name: &str) -> String {
    format!("{}/{}", *BASE_URL, file_name)
}

/// Performs exhaustive checks on the given file path to verify if it needs to be sent as
/// a document.
fn should_send_as_document(file_path: &str) -> bool {
    log::debug!("Checking {}", file_path);
    for file in FORCE_DOCUMENT.split(",") {
        log::debug!("file: {}", file);
        if file == file_path {
            log::debug!("{}: forced as document via env variable", file_path);
            return true;
        }
    }
    if std::fs::metadata(file_path).unwrap().len() > MAX_FILE_SIZE {
        log::debug!("{}: file size is larger than MAX_FILE_SIZE", file_path);
        return true;
    }
    if let Ok(imagesize) = imagesize::size(file_path) {
        if imagesize.height > MAX_DIMEN || imagesize.width > MAX_DIMEN {
            log::debug!("{}: dimensions are larger than MAX_DIMEN", file_path);
            return true;
        };
        if imagesize.width / imagesize.height > 20 {
            log::debug!("{}: dimension ratio is larger than 20", file_path);
            return true;
        }
    };
    false
}

/// Send the given file as a document, with its name and link as caption
fn send_captioned_document(
    cx: Cx,
    file_url: &str,
    file_name: &str,
    file_path: &str,
) -> AutoRequest<MultipartRequest<SendDocument>> {
    let file = if let Some(file_id) = get_remembered_file(file_path) {
        InputFile::FileId(file_id)
    } else {
        InputFile::File(PathBuf::from(file_path))
    };
    cx.answer_document(file)
        .caption(format!(
            "[{}]({})",
            &file_name_to_label(file_name),
            file_url
        ))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(cx.update.id)
}

/// Send the given file as a picture, with its name and link as caption
fn send_captioned_picture(
    cx: Cx,
    file_url: &str,
    file_name: &str,
    file_path: &str,
) -> AutoRequest<MultipartRequest<SendPhoto>> {
    let file = if let Some(file_id) = get_remembered_file(file_path) {
        InputFile::FileId(file_id)
    } else {
        InputFile::File(PathBuf::from(file_path))
    };
    cx.answer_photo(file)
        .caption(format!(
            "[{}]({})",
            &file_name_to_label(file_name),
            file_url
        ))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(cx.update.id)
}

fn remember_file(file_path: String, file_id: String) {
    let hash = get_file_hash(&file_path);
    if let Err(error) = TREE.insert(&format!("{}", hash), file_id.as_str()) {
        log::debug!("failed to insert {} into db: {}", file_id, error);
    };
}

fn get_remembered_file(file_path: &str) -> Option<String> {
    let hash = get_file_hash(&file_path);
    if let Ok(value) = TREE.get(&format!("{}", hash)) {
        if let Some(ivec) = value {
            if let Ok(id) = String::from_utf8(ivec.to_vec()) {
                log::debug!("found id for {}: {}", file_path, id);
                return Some(id);
            }
        };
    };
    None
}

async fn send_random_image(
    cx: Cx,
    images: Vec<String>,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    let file = get_random_file(images);
    let path = get_file_path(&file);
    let link = get_file_url(&file);
    if should_send_as_document(&path) {
        cx.requester
            .send_chat_action(cx.update.chat.id, ChatAction::UploadDocument)
            .await?;
        let msg = send_captioned_document(cx, &link, &file, &path).await?;
        if let Some(doc) = msg.document() {
            let document = doc.clone();
            remember_file(path, document.file_id);
        };
    } else {
        cx.requester
            .send_chat_action(cx.update.chat.id, ChatAction::UploadPhoto)
            .await?;
        let msg = send_captioned_picture(cx, &link, &file, &path).await?;
        if let Some(photos) = msg.photo() {
            let photo = photos[0].clone();
            remember_file(path, photo.file_id);
        };
    }
    Ok(())
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
                    send_random_image(cx, results).await?;
                }
            }
        }
        Command::Random => {
            send_random_image(cx, (*FILES).clone()).await?;
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

async fn run() {
    dotenv().ok();
    teloxide::enable_logging!();

    log::debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env().auto_send();
    teloxide::commands_repl(bot, BOT_NAME.as_str(), answer).await;
}

#[tokio::main]
async fn main() {
    run().await;
}
