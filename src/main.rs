use anyhow::Context;
use dotenv;
use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::model::prelude::ChannelId;
use serenity::prelude::*;
use std::env;
use teloxide::prelude::*;

struct Handler {
    telegram_bot: Bot,
    telegram_channel_id: String,
    discord_channel_id: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(
        &self,
        _context: serenity::client::Context,
        msg: serenity::model::channel::Message,
    ) {
        if msg.channel_id == self.discord_channel_id {
            self.telegram_bot
                .send_message(self.telegram_channel_id.to_owned(), msg.content)
                .await
                .expect("Failed to relay Telegram message");
        }
        ()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let telegram_token = env::var("TELEGRAM_BOT_TOKEN").context("TELEGRAM_BOT_TOKEN not set")?;
    let discord_token = env::var("DISCORD_BOT_TOKEN").context("DISCORD_BOT_TOKEN not set")?;
    let telegram_channel_id =
        env::var("TELEGRAM_CHANNEL_ID").context("TELEGRAM_CHANNEL_ID not set")?;
    let discord_channel_id =
        env::var("DISCORD_CHANNEL_ID").context("DISCORD_CHANNEL_ID not set")?;
    let discord_channel_id = discord_channel_id
        .parse::<u64>()
        .context("DISCORD_CHANNEL_ID is not a valid number")?;

    let bot = Bot::new(telegram_token);

    let framework = StandardFramework::new().configure(|c| c.prefix("~"));
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(discord_token, intents)
        .event_handler(Handler {
            discord_channel_id: ChannelId(discord_channel_id),
            telegram_bot: bot,
            telegram_channel_id,
        })
        .framework(framework)
        .await
        .context("Error creating the Discord client")?;

    client
        .start()
        .await
        .context("Error running the Discord client")?;
    Ok(())
}
