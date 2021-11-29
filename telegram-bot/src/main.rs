use std::error::Error;
use teloxide::{prelude::*, types::ParseMode::MarkdownV2, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Shows details about the bot.")]
    Start,
    #[command(description = "display this text.")]
    Help,
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Start =>{
            cx.reply_to(format!("A bot made in rust by Diffumist Type /help to see what this bot can do")).parse_mode(MarkdownV2).await?
        }

        Command::Help => {
            cx.reply_to(format!(
                "I am a bot made by [Diffumist](https://t.me/Diffumist) in [rust](https://www.rust-lang.org/)
_Here's a list of my commands:_
`/help` _Display this text_
`/start` _Shows bot info_")
            ).disable_web_page_preview(true).parse_mode(MarkdownV2).await?
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
    teloxide::commands_repl(bot, bot_name, answer).await;
}
