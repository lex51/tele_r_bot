use teloxide::{prelude2::*, utils::command::BotCommand};
// use curl::easy::Easy;
// use reqwest::get;
use std::error::Error;
use std::collections::HashMap;

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get current ip")]
    GetIP,
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => bot.send_message(message.chat.id, Command::descriptions()).await?,
        Command::GetIP => {
            let url = "https://api.ipify.org?format=json";
            let response = reqwest::get(url).await?.json::<HashMap<String, String>>().await?;
            // let response = http::handle()
            //     .get(url)
            //     .exec()
            //     .unwrap_or_else(|e| {
            //         panic!("Failed to get {}; error is {}", url, e);
            // });
            bot.send_message(message.chat.id, format!("{:#?}.", response)).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{} and age is {}.", username, age),
            )
            .await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}