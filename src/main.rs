use log::{info, LevelFilter};
use pretty_env_logger::{formatted_builder};
use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    formatted_builder()
        .filter_level(LevelFilter::Info)
        .default_format()
        .init();

    info!("Starting word bot...");

    let bot = Bot::from_env();

    bot.set_my_commands(Command::bot_commands())
        .await
        .expect("Failed to set bot commands");

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
        Command::Info(text) => bot.send_message(msg.chat.id, text).await?,
    };
    Ok(())
}
