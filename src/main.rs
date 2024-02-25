use std::error::Error;
use std::time::Duration;
use log::{error, info, LevelFilter};
use pretty_env_logger::formatted_builder;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{Me, ParseMode};
use tokio::time;
use telegram::Telegram;

use crate::dictionary::DictionaryErrorKind;

mod dictionary;
mod telegram;
mod urban;

#[tokio::main]
async fn main() {
    formatted_builder()
        .filter_level(LevelFilter::Info)
        .default_format()
        .init();

    info!("Starting word bot...");

    let bot = Bot::from_env();

    let mut commands_set = false;
    while !commands_set {
        match bot.set_my_commands(Command::bot_commands()).await {
            Ok(_) => {
                commands_set = true;
                info!("Bot commands set successfully");
            }
            Err(err) => {
                error!("Failed to set bot commands: {:?}: {:?}", err, err.to_string());
                time::sleep(Duration::from_secs(5)).await;
            }
        }
    }

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(message_handler));
    // .branch(Update::filter_callback_query().endpoint(callback_handler))
    // .branch(Update::filter_inline_query().endpoint(inline_query_handler));

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported\\. Without command I will send you the definition of the text that you sent me")]
enum Command {
    #[command(description = "Display this text\\.")]
    Help,
    #[command(description = "Seek the text in the dictionary\\.")]
    Info(String),
    #[command(description = "Seek the text in the urban dictionary\\.")]
    Urban(String),
}

async fn message_handler(
    bot: Bot,
    msg: Message,
    me: Me,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(text) = msg.text() {
        info!("Received message: {:?}", msg);
        match BotCommands::parse(text, me.username()) {
            Ok(Command::Help) => {
                send_large_message(bot.clone(), msg.chat.id, Command::descriptions().to_string()).await?;
            }
            Ok(Command::Info(text)) => {
                send_large_message(bot.clone(), msg.chat.id, build_info_response(&text).await).await?;
            }
            Ok(Command::Urban(text)) => {
                send_large_message(bot.clone(), msg.chat.id, build_urban_response(&text).await).await?;
            }
            Err(_) => {
                if text.starts_with('/') {
                    bot.send_message(msg.chat.id, "Command not found!").await?;
                } else {
                    send_large_message(bot.clone(), msg.chat.id, build_info_response(&text).await).await?;
                }
            }
        }
    }

    Ok(())
}

pub async fn send_large_message(bot: Bot, chat_id: ChatId, message: String) -> Result<(), Box<dyn Error + Send + Sync>> {
    let max_length = 4096;
    let mut message = message;

    while message.len() > max_length {
        if let Some(last_newline) = message[..max_length].rfind('\n') {
            let (chunk, rest) = message.split_at(last_newline);
            bot.send_message(chat_id, chunk.to_string()).parse_mode(ParseMode::MarkdownV2).await?;
            message = rest.to_string();
        } else {
            let (chunk, rest) = message.split_at(max_length);
            bot.send_message(chat_id, chunk.to_string()).parse_mode(ParseMode::MarkdownV2).await?;
            message = rest.to_string();
        }
    }

    if !message.is_empty() {
        bot.send_message(chat_id, message).parse_mode(ParseMode::MarkdownV2).await?;
    }

    Ok(())
}

async fn build_info_response(word: &str) -> String {
    return match dictionary::definition(word).await {
        Ok(defs) => {
            if defs.is_empty() {
                "No definition found".to_string()
            } else {
                defs.to_message()
            }
        }
        Err(e) => {
            let default_message = match e.kind {
                DictionaryErrorKind::NotFound => "No definition found",
                DictionaryErrorKind::Parse => "Error on parse output",
                DictionaryErrorKind::Unknown => "An unknown error occurred"
            };

            let message = if !e.message.is_empty() {
                format!("{}\n`{}`", default_message, e.message)
            } else {
                default_message.to_string()
            };

            message
        }
    };
}

async fn build_urban_response(word: &str) -> String {
    return match urban::definition(word).await {
        Ok(defs) => {
            if defs.is_empty() {
                "No definition found".to_string()
            } else {
                defs.to_message()
            }
        }
        Err(e) => {
            let default_message = match e.kind {
                DictionaryErrorKind::NotFound => "No definition found",
                DictionaryErrorKind::Parse => "Error on parse output",
                DictionaryErrorKind::Unknown => "An unknown error occurred"
            };

            let message = if !e.message.is_empty() {
                format!("{}\n`{}`", default_message, e.message)
            } else {
                default_message.to_string()
            };

            message
        }
    };
}