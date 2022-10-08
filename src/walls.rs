use crate::{
    commands::Command,
    utils::{
        file_name_to_label, get_file_hash, get_random_file, get_search_results, index_pictures,
        join_results_to_string, parse_bool,
    },
};
use once_cell::sync::Lazy;
use std::{env, error::Error, marker::Send, path::PathBuf};
use teloxide::{
    payloads::{
        SendDocument, SendDocumentSetters, SendMessageSetters, SendPhoto, SendPhotoSetters,
    },
    prelude::Requester,
    requests::MultipartRequest,
    types::{ChatAction, InputFile, Message, ParseMode, UserId},
    utils::command::BotCommands,
    Bot,
};
use tracing::debug;

static BASE_URL: Lazy<String> =
    Lazy::new(|| env::var("BASE_URL").expect("BASE_URL must be defined"));
static TREE: Lazy<sled::Db> = Lazy::new(|| sled::open("file_id_cache").unwrap());
pub static BASE_DIR: Lazy<String> =
    Lazy::new(|| env::var("BASE_DIR").expect("BASE_DIR must be defined"));
pub static FILES: Lazy<Vec<String>> = Lazy::new(|| index_pictures(&BASE_DIR));
pub static BOT_OWNER: Lazy<UserId> = Lazy::new(|| {
    let value = env::var("BOT_OWNER_ID").expect("BOT_OWNER_ID must be defined");
    let id = value
        .parse::<u64>()
        .expect("BOT_OWNER_ID must be a valid integer");
    UserId(id)
});

/// Telegram mandates a photo can not be larger than 10 megabytes
const MAX_FILE_SIZE: u64 = 10_485_760;

/// Telegram mandates a photo can not be longer than 10000 pixels across any dimension
const MAX_DIMEN: usize = 10000;

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

fn basename(file_name: &str) -> String {
    file_name.replace(&format!("{}/", *BASE_DIR), "")
}

/// Performs exhaustive checks on the given file path to verify if it needs to be sent as
/// a document.
fn should_send_as_document(file_path: &str) -> bool {
    let file_name = basename(file_path);
    if std::fs::metadata(file_path).unwrap().len() > MAX_FILE_SIZE {
        debug!("{}: file size is larger than MAX_FILE_SIZE", file_name);
        return true;
    }
    if let Ok(imagesize) = imagesize::size(file_path) {
        if imagesize.height + imagesize.width > MAX_DIMEN {
            debug!("{}: dimensions are larger than MAX_DIMEN", file_name);
            return true;
        };
        if imagesize.width / imagesize.height > 20 {
            debug!("{}: dimension ratio is larger than 20", file_name);
            return true;
        }
    };
    false
}

/// Send the given file as a document, with its name and link as caption
fn send_captioned_document(
    bot: &Bot,
    message: &Message,
    file_url: &str,
    file_name: &str,
    file_path: &str,
) -> MultipartRequest<SendDocument> {
    let file = if let Some(file_id) = get_remembered_file(file_path) {
        InputFile::file_id(file_id)
    } else {
        InputFile::file(PathBuf::from(file_path))
    };
    bot.send_document(message.chat.id, file)
        .caption(format!(
            "[{}]({})",
            &file_name_to_label(file_name),
            file_url
        ))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(message.id)
}

/// Send the given file as a picture, with its name and link as caption
fn send_captioned_picture(
    bot: &Bot,
    message: &Message,
    file_url: &str,
    file_name: &str,
    file_path: &str,
) -> MultipartRequest<SendPhoto> {
    let file = if let Some(file_id) = get_remembered_file(file_path) {
        InputFile::file_id(file_id)
    } else {
        InputFile::file(PathBuf::from(file_path))
    };
    bot.send_photo(message.chat.id, file)
        .caption(format!(
            "[{}]({})",
            &file_name_to_label(file_name),
            file_url
        ))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(message.id)
}

fn remember_file(file_path: &str, file_id: &str) {
    let hash = get_file_hash(file_path);
    if let Err(error) = TREE.insert(&format!("{}", hash), file_id) {
        debug!("Failed to insert {} into db: {}", file_id, error);
    };
}

fn get_remembered_file(file_path: &str) -> Option<String> {
    let hash = get_file_hash(file_path);
    if let Ok(Some(ivec)) = TREE.get(&format!("{}", hash)) {
        if let Ok(id) = String::from_utf8(ivec.to_vec()) {
            let file_name = basename(file_path);
            debug!("Found id for {}: {}", file_name, id);
            return Some(id);
        }
    };
    None
}

async fn send_random_image(
    bot: &Bot,
    message: &Message,
    images: Vec<String>,
) -> Result<(), Box<dyn Error + Sync + Send + 'static>> {
    let file = get_random_file(&images);
    let path = get_file_path(&file);
    let link = get_file_url(&file);
    if should_send_as_document(&path) {
        bot.send_chat_action(message.chat.id, ChatAction::UploadDocument)
            .await?;
        let msg = send_captioned_document(bot, message, &link, &file, &path).await?;
        if let Some(doc) = msg.document() {
            let document = doc.clone();
            remember_file(&path, &document.file.id);
        };
    } else {
        bot.send_chat_action(message.chat.id, ChatAction::UploadPhoto)
            .await?;
        let msg = send_captioned_picture(bot, message, &link, &file, &path).await?;
        if let Some(photos) = msg.photo() {
            let photo = photos[0].clone();
            remember_file(&path, &photo.file.id);
        };
    }
    Ok(())
}

pub(crate) async fn handler(
    bot: Bot,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_chat_action(message.chat.id, ChatAction::Typing)
                .await?;
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?;
        }
        Command::Pic { search_term } => {
            if search_term.is_empty() {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "No search query passed")
                    .reply_to_message_id(message.id)
                    .await?;
            } else {
                let results = search(&search_term.replace(' ', "_"));
                if results.is_empty() {
                    bot.send_chat_action(message.chat.id, ChatAction::Typing)
                        .await?;
                    bot.send_message(
                        message.chat.id,
                        format!("No picture found for '{}'", search_term),
                    )
                    .reply_to_message_id(message.id)
                    .await?;
                } else {
                    send_random_image(&bot, &message, results).await?;
                }
            }
        }
        Command::Random => {
            send_random_image(&bot, &message, (*FILES).clone()).await?;
        }
        Command::Search { search_term } => {
            bot.send_chat_action(message.chat.id, ChatAction::Typing)
                .await?;
            let res = search(&search_term);
            if res.is_empty() {
                bot.send_message(
                    message.chat.id,
                    format!("No results found for '{}'", search_term),
                )
                .reply_to_message_id(message.id)
                .await?;
            } else {
                bot.send_message(
                    message.chat.id,
                    join_results_to_string(&search_term, &res, &BASE_URL),
                )
                .parse_mode(ParseMode::MarkdownV2)
                .disable_web_page_preview(true)
                .reply_to_message_id(message.id)
                .await?;
            }
        }
        Command::Ddinstagram { filter_state } => {
            if let Some(_) = message.from().map(|from| from.id != *BOT_OWNER) {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            } else {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        crate::ddinstagram::set_filter_state(bot, message, filter_state).await?;
                    }
                    Err(error_message) => {
                        bot.send_chat_action(message.chat.id, ChatAction::Typing)
                            .await?;
                        bot.send_message(message.chat.id, error_message)
                            .reply_to_message_id(message.id)
                            .await?;
                    }
                }
            }
        }
        Command::Vxtwitter { filter_state } => {
            if let Some(_) = message.from().map(|from| from.id != *BOT_OWNER) {
                bot.send_chat_action(message.chat.id, ChatAction::Typing)
                    .await?;
                bot.send_message(message.chat.id, "You are not authorized for this action")
                    .reply_to_message_id(message.id)
                    .await?;
            } else {
                match parse_bool(&filter_state) {
                    Ok(filter_state) => {
                        crate::vxtwitter::set_filter_state(bot, message, filter_state).await?;
                    }
                    Err(error_message) => {
                        bot.send_chat_action(message.chat.id, ChatAction::Typing)
                            .await?;
                        bot.send_message(message.chat.id, error_message)
                            .reply_to_message_id(message.id)
                            .await?;
                    }
                }
            }
        }
    };
    Ok(())
}
