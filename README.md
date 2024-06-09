# Rusty Embed Fixer Bot
Simple discord embed fixer bot that can also provide download links. Written in rust using serenity.

I don't host it except for my own discord server, but I do provide prebuilt binaries so you can host it yourself easily.

## How to Use The Bot
### Theres two methods:

1. Say anything

- As long as you have tiktok/instagram/twitter/x link(s) in your message, the bot will try to resend it. It won't re-reply or re-upload your attachment though. For tiktok slideshows, refer to the third command from below

2. Actual commands

- qdev
  - returns version & dev name
- qsay [something]
  - make the bot say whatever you want (prebuilt does not have this command)
- q[link] 
  - If used with tiktok slideshow link, will send all image links in chunks of 5 (this embeds)
  - If used with tiktok video link, will give download link (this doesn't embed). Just paste normally without prefix if you want the embed.
  - If used with instagram reel link, will give download link (this embeds)
  - Doesn't work well with instagram slideshows, as instafix only returns the first 4 images and is manipulated as a grid
  - Doesnt support anything twitter related yet. Not planning to.

## How to get the bot
### Prebuilt
For those who don't even know what Rust is, go with this instead!
1. Download this https://github.com/yuvlian/rusty_embed_fixer_bot/releases/download/1.2.0/rusty_embed_fixer_bot.7z
2. Extract the zip
3. Edit config.yml to your actual discord bot token
4. Launch the binary (aka .exe)

   (Yes I know it's named "rusty_tiktok_bot" despite supporting 2 other websites)
5. As long as the binary is running, bot is online

### Build from source
I'm sure you already know how if you're planning to build from source

### Special thanks
- folks behind serenity, tnktok.com, ddinstagram.com, vxtwitter.com, fixvx.com 
- https://api.tiklydown.eu.org
- xeondev