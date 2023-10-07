## discord-telegram-simple-relay

A simple service written in Rust which listens to messages on a Discord channel and relay them to a Telegram chat.

### Instructions

Set these env vars:
```
TELEGRAM_BOT_TOKEN=
DISCORD_BOT_TOKEN=
TELEGRAM_CHANNEL_ID=
DISCORD_CHANNEL_ID=
```

and

```
cargo run
```

### TELEGRAM_BOT_TOKEN

Follow [how to create a Telegram bot](https://core.telegram.org/bots#3-how-do-i-create-a-bot)

### DISCORD_BOT_TOKEN

Follow [how to create a Discord bot](https://discordjs.guide/preparations/setting-up-a-bot-application.html)

### TELEGRAM_CHANNEL_ID

Open your channel via web, check the URL after the `#`. 
It should start with -100.
```https://web.telegram.org/a/#-100123456789```

### DISCORD_CHANNEL_ID

Right click on any message in the channel and `Copy Link`.
It's a `/` separated list of 3 IDs.
Use the second one.
```
https://discord.com/channels/1234567890123456789/1234567890123456789/1234567890123456789
```

### LICENSE

MIT