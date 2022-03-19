use teloxide::{prelude2::*, utils::command::BotCommand};
use std::error::Error;
use std::collections::HashMap;
use serde_json::json;
use sysinfo::{
    // NetworkExt, NetworksExt, ProcessExt, 
    System, SystemExt};
use sysinfo::{DiskExt};
    

#[derive(BotCommand, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "get current ip")]
    GetIP,
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
    #[command(description = "get hw info")]
    GetInfo,
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
            bot.send_message(message.chat.id, format!("{}", json!(response)["ip"].to_string())).await?
        }

        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{} and age is {}.", username, age),
            )
            .await?
        }
        Command::GetInfo => {
            let sys = System::new_all();
            let mut hw_info = String::new();
            for disk in sys.disks() {
                // println!("free {:?}% of {:?}Gb", ((disk.available_space()*100)/disk.total_space()), disk.total_space()/1024/1024/1024);
                hw_info.push_str(&format!("free {:?}% of {:?}Gb", ((disk.available_space()*100)/disk.total_space()), disk.total_space()/1024/1024/1024).to_string());
                hw_info.push_str("\n");
            }

            hw_info.push_str(&format!("RAM used {}% from {}GB and {}% swap", (sys.used_memory()*100)/sys.total_memory(), sys.total_memory()/1024/1024, (sys.used_swap()*100)/sys.total_swap()).to_string());

            bot.send_message(message.chat.id, hw_info).await?
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