mod commands;
mod utils;

use dotenv::dotenv;

use teloxide::types::{InputFile, ParseMode};
use teloxide::{prelude::*, utils::command::BotCommand};

use lazy_static::lazy_static;
use std::env;

use crate::commands::Command;
use crate::utils::{
    file_name_to_label, get_random_file, get_search_results, index_pictures, join_results_to_string,
};
use teloxide::requests::{SendChatActionKind, SendPhoto};

lazy_static! {
    static ref FILES: Vec<String> = index_pictures(&**BASE_DIR);
    static ref BASE_URL: String = env::var("BASE_URL").expect("BASE_URL must be defined");
    static ref BASE_DIR: String = env::var("BASE_DIR").expect("BASE_DIR must be defined");
    static ref BOT_NAME: String = env::var("BOT_NAME").unwrap_or_default();
}

fn search(search_term: &str) -> Vec<String> {
    get_search_results((*FILES).clone(), search_term)
}

fn send_captioned_picture(
    cx: &DispatcherHandlerCx<Message>,
    link: String,
    file: &str,
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
) -> ResponseResult<()> {
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
                let results = search(&args.join("_"));
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
                    send_captioned_picture(&cx, link, &file).send().await?
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
            send_captioned_picture(&cx, link, &file).send().await?
        }
        Command::Search => {
            let search_term = args.join("_");
            let res = search(&search_term);
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

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    rx.commands::<Command, &str>(&BOT_NAME)
        .for_each_concurrent(None, |(cx, command, args)| async move {
            answer(cx, command, &args).await.log_on_error().await;
        })
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    dotenv().ok();
    teloxide::enable_logging!();

    log::debug!("Indexed {} files", FILES.len());

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(handle_commands)
        .dispatch()
        .await;
}
