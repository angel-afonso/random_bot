mod action;

use action::*;
use lazy_static::lazy_static;
use std::borrow::Cow;
use teloxide::{prelude::*, types::InputFile};

macro_rules! input_file {
    ($name: literal, $path:literal) => {
        InputFile::Memory {
            file_name: $name.into(),
            data: Cow::Borrowed(include_bytes!($path)),
        }
    };
}

lazy_static! {
    static ref ACTIONS: Vec<Action> = {
        vec![
            Action {
                condition: |message| message.contains("13"),
                consequence: ActionType::Photo(input_file!(
                    "trap card.jpg",
                    "images/magic_number_trap.jpg"
                )),
            },
            Action {
                condition: |message| message.contains("todo cool todo bien todo fine"),
                consequence: ActionType::Text("todo fine todo bien todo cool"),
            },
        ]
    };
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting random_bot...");

    let bot = Bot::from_env()
        .auto_send()
        .parse_mode(teloxide::types::ParseMode::MarkdownV2);

    teloxide::repl(bot, |message| async move {
        for action in ACTIONS.iter() {
            if (action.condition)(message.update.text().unwrap_or("")) {
                match action.consequence.clone() {
                    ActionType::Photo(file) => {
                        message.reply_photo(file).await?;
                    }
                    ActionType::Text(text) => {
                        message.reply_to(text).await?;
                    }
                }
                break;
            }
        }

        respond(())
    })
    .await;
}

#[tokio::main]
async fn main() {
    run().await;
}
