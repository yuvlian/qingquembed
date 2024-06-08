# Rusty Embed Fixer Bot
Simple bot that resends your message that contains tiktok/instagram/twitter/x link(s) with the embed fixed. Written in rust using serenity.

I don't host it except for my own discord server, but I do provide prebuilt binaries so you can host it yourself easily.

## How to Use The Bot
### Theres two methods:

1. Say anything

- As long as you have tiktok/instagram/twitter/x link(s) in your message, the bot will try to resend it. It won't re-reply or re-upload your attachment though. For tiktok slideshows, refer to the third command from below

2. Actual commands

- qdev -> returns version & dev name
- qsay [something] -> make the bot say whatever you want (prebuilt does not have this command)
- q[tiktok link] -> posts image links from tiktok slideshow in chunks of 5. if you use this with a tiktok video link instead, it will give you download link

## How to get the bot
### Prebuilt
For those who don't even know what Rust is, go with this instead!
1. Download this https://github.com/yuvlian/rusty_embed_fixer_bot/releases/download/1.1.0/rusty_embed_fixer_bot.7z
2. Extract the zip
3. Edit config.yml to your actual discord bot token
4. Launch the binary (aka .exe)

   (Yes I know it's named "rusty_tiktok_bot" despite supporting 2 other websites)
5. Bot is online if you didn't mess anything up.

### Build from source
I'm sure you already know how if you're planning to build from source

### Special thanks
- folks behind serenity, tnktok.com, ddinstagram.com, vxtwitter.com, fixvx.com 
- https://api.tiklydown.eu.org
- xeondev