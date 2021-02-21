use std::error::Error;
use reqwest;
use teloxide::{prelude::*, types::ParseMode::MarkdownV2, utils::command::BotCommand};

pub async fn pixiv_search(query: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://api.pixiv.net/v1/search/illust?word={}&per_page=1", query);
    let res = client.get(&url).send()?.text()?;
    let json: serde_json::Value = serde_json::from_str(&res)?;
    let url = json["response"]["docs"][0]["urls"]["original"].as_str().unwrap();
    Ok(url.to_string())
}
pub async fn download_image(url: &str) -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let res = client.get(url).send()?;
    let mut file = std::fs::File::create("image.png")?;
    res.copy_to(&mut file)?;
    Ok(())
}

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Shows details about the bot.")]
    Start,
    #[command(description = "display this text.")]
    Help,
}

async fn message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Start =>{
            cx.reply_to(format!("A bot made in rust by Diffumist Type /help to see what this bot can do")).parse_mode(MarkdownV2).await?
        }

        Command::Help => {
            cx.reply_to(format!()).disable_web_page_preview(true).parse_mode(MarkdownV2).await?
        }

    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Dmist Bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "Dmist_bot".to_string();
    teloxide::commands_repl(bot, bot_name, message).await;
}
