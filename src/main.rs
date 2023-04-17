use std::time::Duration;
use log::{error, info, LevelFilter};
use pretty_env_logger::formatted_builder;
use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::ParseMode;
use tokio::time;
use telegram::Telegram;

use crate::dictionary::DictionaryErrorKind;

mod dictionary;
mod telegram;

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
                error!("Failed to set bot commands: {:?}", err);
                time::sleep(Duration::from_secs(5)).await;
            }
        }
    }
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported. Without command I will send you the definition of the text that you sent me")]
enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Seek the text in the dictionary.")]
    Info(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    info!("Received message: {:?}", msg);
    info!("Parsed command: {:?}", cmd);
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Info(word) => bot.send_message(msg.chat.id, build_info_response(word.as_str()).await).parse_mode(ParseMode::MarkdownV2).await?,
    };
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
            match e.kind {
                DictionaryErrorKind::NotFound => "No definition found".to_string(),
                DictionaryErrorKind::Parse => "Error on parse output".to_string(),
                DictionaryErrorKind::Unknown => "An unknown error occurred".to_string()
            }
        }
    };
}
