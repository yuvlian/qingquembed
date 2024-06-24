# Qingquembed [1.3.0]
Qingquembed is a Discord bot that was written in Rust using serenity.

Its purpose is to fix embed links of TikTok, Instagram, Twitter/X. It can also provide download links. In the next version, I will add support for YouTube downloads.

Why Rust, you may ask? Cause it's so damn fast, safe, and the binary is small and independent.

The bot is named after the best girl from Honkai: Star Rail, Qingque.

I only host this bot for myself, but I provide Windows ~~and~~ ~~Linux~~ prebuilts so you can host it yourself easily.

## Bot Usage
### There are two methods:

**1. Say anything**
  - The bot will use regex to find and replace the part of your message that contains TikTok/Instagram/Twitter/X URL and then resends it. Keep in mind it won't re-reply nor will it reupload the attachment of the message.
  - Tiktok URLs will be replaced with VxTiktok, Instagram URLs with InstaFix, Twitter/X URLs with VxTwitter.
  - For slideshows (e.g. TikTok slideshows), please refer to the actual commands number 3.

**2. Actual commands**
  - **qdev**
    - Will send information about the bot version and developer.
  - **qsay [string]**
    - Will make the bot say something. Does not reply nor reupload attachments. This is completely removed in the prebuilt.
  - **q[link]**
    - If used with a TikTok slideshow link, it will give you embed URL of the images in chunks of 5.
    - If used with a TikTok video link, it will give you the download link (no watermark). This link does not embed.
    - If used with an Instagram video link (or reel), it will give you a download link that embeds. However, due to Discord's proxies, you can only play the video by opening it in browser or downloading it. Simply paste the Instagram link normally, if you only plan to play the video on Discord.
    - This command does not work well with Instagram slideshows, due to InstaFix returning only the first four images as a grid, and will break if it's a video and image slideshow.
    - Does not support Twitter/X yet. I am not planning to add support for it as I don't use it. 

## How To Get The Bot
### Prebuilt:
1. Download the latest [qingquembed.zip](https://github.com/yuvlian/qingquembed/releases).
2. Extract the zip.
3. Replace the token in config.yml with your discord bot token.
4. Run the binary. 

**Note:** Currently only has Windows prebuilt binary. Can't compile for Linux due to some linking bullshit on NixOS, and I'm not gonna bother.

### Build from source:
I'm sure you already know how to if you are planning to build from source. If you don't even know what Rust is, go with prebuilt.

## Special thanks to:
- Folks behind Rust and the dependencies of this bot, especially Serenity.
- Folks behind VxTiktok, VxTwitter, and InstaFix.
- Folks behind https://tiklydown.eu.org.
- And lastly, [thexeondev](https://github.com/thexeondev) for giving feedbacks on my code.