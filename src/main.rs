use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
};
use regex::Regex;
use std::sync::LazyLock;
use serde::Deserialize;
use std::{
    fs::File,
    io::BufReader,
};
mod libtok; // import libtok.rs
use libtok::TikTokApiResponse;

struct Handler;

// Regex to match links
static TIKTOK_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https?://(www\.)?(tiktok\.com|vt\.tiktok\.com|vm\.tiktok\.com)/?").unwrap());
static INSTAGRAM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https?://(www\.)?(instagram\.com|instagr\.am)/?").unwrap());
static TWITTER_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https?://(www\.)?(twitter\.com)/?").unwrap());
static X_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"https?://(www\.)?(x\.com)/?").unwrap());

// Static strings to replace the matched stuff
static TNKTOK: LazyLock<String> = LazyLock::new(|| String::from("https://tnktok.com/"));
static DDINSTAGRAM: LazyLock<String> = LazyLock::new(|| String::from("https://ddinstagram.com/"));
static VXTWITTER: LazyLock<String> = LazyLock::new(|| String::from("https://vxtwitter.com/"));
static FIXVX: LazyLock<String> = LazyLock::new(|| String::from("https://fixvx.com/"));

// Allowed users for "qsay" command
static USER1: LazyLock<String> = LazyLock::new(|| String::from("876725552474644490"));

// Prefixes
static PREFIX_QSAY: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\bqsay\b").unwrap());
static PREFIX_QDEV: LazyLock<String> = LazyLock::new(|| String::from("qdev"));
static PREFIX_Q: LazyLock<String> = LazyLock::new(|| String::from("q"));

// tiklydown.eu.org API Base URL
static API_TIKLYDOWN_EU_ORG: LazyLock<String> = LazyLock::new(|| String::from("https://api.tiklydown.eu.org/api/download/v2?url="));

// Version & Dev of Bot
static DEV: LazyLock<String> = LazyLock::new(|| String::from("[version]\nrust = \"1.80.0-nightly\"\nserenity = \"0.12.2\"\nthis_bot = \"1.1.0\"\n\n[source]\ndev = \"yulian\"\nrepo = \"<https://github.com/yuvlian/rusty_embed_fixer_bot>\"\nlicense = \"bsd-3-clause\""));

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // Prevent bot loops.
        if msg.author.bot {
            return;
        }

        // Version check & dev.
        // Example usage: qdev
        if msg.content == *PREFIX_QDEV {
            if let Err(why) = msg.channel_id.say(&ctx.http, &*DEV).await {
                println!("Error sending message: {why:?}");
            }
        }
        
        // Gets author id
        let allowed_user = &msg.author.id.to_string();

        // Makes the bot say something. Only allowed for certain users ;)
        // Example usage: qsay Hello World!
        // This is disabled in the prebuilt.
        // Be sure to change or remove this to your liking if you're building from source!
        // Otherwise... I will prank you! ^w^
        if PREFIX_QSAY.is_match(&msg.content) && &*USER1 == allowed_user {
            
            // Bot sends the message
              if let Err(why) = msg.channel_id.say(&ctx.http, PREFIX_QSAY.replace_all(&msg.content, "")).await {
                  println!("Error sending message: {:?}", why);
             }

            // Bot deletes the original message
              if let Err(why) = msg.delete(&ctx.http).await {
                 println!("Error deleting message: {:?}", why);
             }
          }

        // TikTok Scraper. Sends image embed links if slideshow, sends download link to video if video. 
        // You can just paste without adding "q" before the link if you want to play a video.
        // Example usage: qhttps://www.tiktok.com/@xalbierblx/video/7356560922002951456
        if msg.content.starts_with(&*PREFIX_Q) && TIKTOK_REGEX.is_match(&msg.content) {
            // Notify that the bot detects the message.
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Processing TikTok link: <{}>", &msg.content[1..])).await {
                println!("Error sending message: {why:?}");
            }

            // Formats base url + url from message
            let url = format!("{}{}", *API_TIKLYDOWN_EU_ORG, &msg.content[1..]); 

            // Handle URL result from API
            match TikTokApiResponse::from_url(&url).await {
                Ok(api_response) => {

                    // If success
                    let media_urls = api_response.get_media_urls();
                    if !media_urls.is_empty() {
                        println!("Media URLs found: {:?}", media_urls);

                        // Chunks so that it sends up to 5 media links per message
                        for chunk in media_urls.chunks(5) {

                            // Create new message
                            let mut message = String::new();

                            // Format and append each media link to the message
                            for (index, media_url) in chunk.iter().enumerate() {
                                let content_index = index + 1; 
                                let formatted_content = format!("[Content {}]({}) ", content_index, media_url);
                                message.push_str(&formatted_content);
                            }

                            // Send formatted message
                            if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    } else {

                        // If fail to find media
                        println!("No media URLs found.");
                        if let Err(why) = msg.channel_id.say(&ctx.http, "No media URL found. Are you sure the TikTok link is valid?").await {
                            println!("Error sending no media URL found message: {:?}", why);
                        }
                    }
                }

                // If fetching fails
                Err(why) => {
                    println!("Error fetching TikTok data: {:?}", why);
                    if let Err(err_msg) = msg.channel_id.say(&ctx.http, format!("Failed to fetch TikTok data: {:?}", why)).await {
                        println!("Error sending failure message: {:?}", err_msg);
                    }
                }
            }
        }

        // Prefixless stuff.
        // This will automatically resend your messages that contains a tiktok/instagram/twitter/x link 
        // The resent message will have said links replaced with the embed fixed links
        // Note: It does not re-reply or reupload attachments
        if !msg.content.starts_with(&*PREFIX_Q) && TIKTOK_REGEX.is_match(&msg.content) || INSTAGRAM_REGEX.is_match(&msg.content) || TWITTER_REGEX.is_match(&msg.content) || X_REGEX.is_match(&msg.content) {
            
            // Replace the links
            let tiktok_replace = TIKTOK_REGEX.replace_all(&msg.content, &*TNKTOK);
            let instagram_replace = INSTAGRAM_REGEX.replace_all(&tiktok_replace, &*DDINSTAGRAM);
            let twitter_replace = TWITTER_REGEX.replace_all(&instagram_replace, &*VXTWITTER);
            let x_replace = X_REGEX.replace_all(&twitter_replace, &*FIXVX);
            
            // Add author to tell who sent the original message
            let replaced_content = format!("[<@{}>] {}", msg.author.id, x_replace);
            
            // Send modified message
            if let Err(why) = msg.channel_id.say(&ctx.http, replaced_content).await {
                println!("Error sending message: {:?}", why);
            }
            
            // Delete the original message
            if let Err(why) = msg.delete(&ctx.http).await {
                println!("Error deleting message: {:?}", why);
            }
        }
    }
}

// config.yml Deserialization
#[derive(Deserialize)]
struct Config {
    token: String,
}

// Read config.yml for the token
static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let file = File::open("config.yml").expect("Config file not found");
    let reader = BufReader::new(file);
    serde_yml::from_reader(reader).expect("Error reading config")
});

// Gateway intents. This will make the bot read messages from servers and direct messages.
static INTENTS: LazyLock<GatewayIntents> = LazyLock::new(|| {
    GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT
});

#[tokio::main]
async fn main() {
    println!("{}\n", *DEV);
    
    let mut client =
        Client::builder(&*CONFIG.token, *INTENTS).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}