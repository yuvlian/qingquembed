use std::{
    fs::File,
    io::BufReader,
    sync::LazyLock,
};
use serde::Deserialize;
use regex::Regex;
use reqwest::Client as ReqClient;
use serenity::{
    async_trait,
    model::channel::Message,
    prelude::*,
};
mod libtok; // import libtok.rs
use libtok::TikTokApiResponse;

// Regex to match links
static TIKTOK_REGEX: LazyLock<Regex> = LazyLock::new(|| 
    Regex::new(r"https?://(www\.)?(tiktok\.com|vt\.tiktok\.com|vm\.tiktok\.com)/?").unwrap()
);
static INSTAGRAM_REGEX: LazyLock<Regex> = LazyLock::new(|| 
    Regex::new(r"https?://(www\.)?(instagram\.com|instagr\.am)/?").unwrap()
);
static TWITTER_REGEX: LazyLock<Regex> = LazyLock::new(|| 
    Regex::new(r"https?://(www\.)?(twitter\.com)/?").unwrap()
);
static X_REGEX: LazyLock<Regex> = LazyLock::new(|| 
    Regex::new(r"https?://(www\.)?(x\.com)/?").unwrap()
);

// Static strings to replace the matched stuff
static VXTIKTOK: LazyLock<String> = LazyLock::new(|| 
    String::from("https://vt.vxtiktok.com/")
);
static DDINSTAGRAM: LazyLock<String> = LazyLock::new(|| 
    String::from("https://ddinstagram.com/")
);
static VXTWITTER: LazyLock<String> = LazyLock::new(|| 
    String::from("https://vxtwitter.com/")
);
static FIXVX: LazyLock<String> = LazyLock::new(|| 
    String::from("https://fixvx.com/")
);
static D_DDINSTAGRAM: LazyLock<String> = LazyLock::new(|| 
    String::from("https://d.ddinstagram.com/")
);

// Reqwest Client for d.ddinstagram thing
static T_BOT_CLIENT: LazyLock<ReqClient> = LazyLock::new(|| 
    ReqClient::builder().user_agent("TelegramBot").build().unwrap()
);

// Allowed user for "qsay" command
static USER_1: LazyLock<String> = LazyLock::new(|| 
    String::from("932589665230401566")
);

// Actual commands
static PREFIX_QSAY: LazyLock<String> = LazyLock::new(|| 
    String::from("qsay")
);
static PREFIX_QDEV: LazyLock<String> = LazyLock::new(|| 
    String::from("qdev")
);
static PREFIX_Q: LazyLock<String> = LazyLock::new(|| 
    String::from("qhttp") // Added "http" incase someone says stupid shit like quantum [link]
); 

// Version & Dev of Bot
fn dev_info() -> String {
String::from(
r#"[version]
rust = "1.81.0-nightly"
serenity = "0.12.2"
qingquembed = "1.3.0"

[source]
dev = "yulian"
repo = "<https://github.com/yuvlian/rusty_embed_fixer_bot>"
license = "bsd-3-clause"
"#)
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {

        // Prevent bot loops.
        if msg.author.bot {
            return;
        }

        // Get user_id
        let user_id = msg.author.id;

        // Version check & dev.
        // Example usage: qdev
        if msg.content == *PREFIX_QDEV {
            if let Err(why) = msg.channel_id.say(&ctx.http, 
                dev_info()
            ).await {
                println!("Error sending message: {:?}", why);
            }
        }
        

        // Makes the bot say something. Whitelist is hardcoded.
        // Example usage: qsay Hello World!
        // This (and anything related to it) is removed in the prebuilt.
        if msg.content.starts_with(&*PREFIX_QSAY) {
            let allowed_user = user_id.to_string();
            if allowed_user == *USER_1 {
                // Bot deletes the original message
                if let Err(why) = msg.delete(&ctx.http).await {
                    println!("Error deleting message: {:?}", why);
                }

                // Bot sends the message
                if let Err(why) = msg.channel_id.say(&ctx.http,
                    msg.content.replace(&*PREFIX_QSAY, "")
                ).await {
                    println!("Error sending message: {:?}", why);
                }
            }
        }

        // TikTok Scraper. Sends image embed links if slideshow, sends download link to video if video. 
        // You can just paste without adding "q" before the link if you want to play a video.
        // Example usage: qhttps://www.tiktok.com/@xalbierblx/video/7356560922002951456
        if msg.content.starts_with(&*PREFIX_Q) && TIKTOK_REGEX.is_match(&msg.content) {
            // Deletes original message
            if let Err(why) = msg.delete(&ctx.http).await {
                println!("Error deleting message: {:?}", why);
            }

            // Notify that the bot detects the message.
            if let Err(why) = msg.channel_id.say(&ctx.http,
                format!("[<@{}>] Processing TikTok link: <{}>", user_id, &msg.content[1..])
            ).await {
                println!("Error sending message: {:?}", why);
            }

            // Formats base url + url from message
            let url = format!("https://api.tiklydown.eu.org/api/download/v2?url={}", &msg.content[1..]); 

            // Handle URL result from API
            match TikTokApiResponse::from_url(&url).await {
                Ok(api_response) => {
                    // If success
                    let media_urls = api_response.get_media_urls();
                    if !media_urls.is_empty() {
                        // Log the URL
                        println!("Media URLs found: {:?}", media_urls);

                        // Chunks so that it sends up to 5 media links per message
                        for chunk in media_urls.chunks(5) {
                            // Create new message
                            let mut message = String::new();
                            // Format and append each media link to the message
                            for (index, media_url) in chunk.iter().enumerate() {
                                let content_index = index + 1;
                                let formatted_content = format!(
                                    "[Content {}]({}) ", content_index, media_url
                                );
                                message.push_str(&formatted_content);
                            }

                            // Send formatted message
                            if let Err(why) = msg.channel_id.say(&ctx.http, 
                                message
                            ).await {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    } else {
                        // If fail to find media
                        println!("No media URLs found.");

                        // Inform the user
                        if let Err(why) = msg.channel_id.say(&ctx.http,
                            "No media URL found. Are you sure the TikTok link is valid?"
                        ).await {
                            println!("Error sending message: {:?}", why);
                        }
                    }
                }

                // If fetching fails
                Err(why) => {
                    println!("Error fetching TikTok data: {:?}", why);

                    // Inform the user
                    if let Err(err_msg) = msg.channel_id.say(&ctx.http,
                        format!("Failed to fetch TikTok data: {:?}", why)
                    ).await {
                        println!("Error sending message: {:?}", err_msg);
                    }
                }
            }
        }


        // Instagram scraper
        // Doesn't work well with slideshows, as it depends on InstaFix, 
        // which will return up to 4 images at once (and they are manipulated to be singular grid image)
        // Good for downloading singular image or reel/video though.
        // Usage example: qhttps://instagram.com/reel/C53oGPULGk0/
	    if msg.content.starts_with(&*PREFIX_Q) && INSTAGRAM_REGEX.is_match(&msg.content) {
            // Deletes original message
            if let Err(why) = msg.delete(&ctx.http).await {
                println!("Error deleting message: {:?}", why);
            }

            // Notify that the bot detects the message
            if let Err(why) = msg.channel_id.say(&ctx.http,
                format!("[<@{}>] Processing Instagram link: <{}>", user_id, &msg.content[1..])
            ).await {
                println!("Error sending message: {why:?}");
            }

            // convert original url to d.ddinstagram
            let d_dd_url = INSTAGRAM_REGEX.replace_all(&msg.content[1..], &*D_DDINSTAGRAM);

            // Start fetching response
            match T_BOT_CLIENT.get(&*d_dd_url).send().await {
                // If success
                Ok(response) => {
                    let response_url = response.url().to_string();

                    println!("Media URL: {}", response_url);
                    // Send response to discord
                    if let Err(why) = msg.channel_id.say(&ctx.http,
                        format!("[Download]({})", response_url)
                    ).await {
                        println!("Error sending message: {:?}", why);
                    }
                }
                // If fail
                Err(why) => {
                    println!("Error fetching URL: {:?}", why);
                    if let Err(err_msg) = msg.channel_id.say(&ctx.http,
                        format!("Failed to fetch Instagram data: {:?}", why)
                    ).await {
                        println!("Error sending failure message: {:?}", err_msg);
                    }
                }
            }
            return;
        }

        // Prefixless stuff.
        // This will automatically resend your messages that contains a tiktok/instagram/twitter/x link 
        // The resent message will have said links replaced with the embed fixed links
        // Note: It does not re-reply or reupload attachments
        if !msg.content.starts_with(&*PREFIX_Q) && TIKTOK_REGEX.is_match(&msg.content) 
            || INSTAGRAM_REGEX.is_match(&msg.content) 
            || TWITTER_REGEX.is_match(&msg.content) 
            || X_REGEX.is_match(&msg.content) {
            
            // Delete the original message
            if let Err(why) = msg.delete(&ctx.http).await {
                println!("Error deleting message: {:?}", why);
            }

            // Replace the links
            let tiktok_replace = TIKTOK_REGEX.replace_all(&msg.content, &*VXTIKTOK);
            let instagram_replace = INSTAGRAM_REGEX.replace_all(&tiktok_replace, &*DDINSTAGRAM);
            let twitter_replace = TWITTER_REGEX.replace_all(&instagram_replace, &*VXTWITTER);
            let x_replace = X_REGEX.replace_all(&twitter_replace, &*FIXVX);
            
            // Add author to tell who sent the original message
            let replaced_content = format!("[<@{}>] {}", user_id, x_replace);

            // Send modified message
            if let Err(why) = msg.channel_id.say(&ctx.http, 
                replaced_content
            ).await {
                println!("Error sending message: {:?}", why);
            }
            
        }
    }
}


#[derive(Deserialize)]
struct Config {
    token: String,
}

#[tokio::main]
async fn main() {
    let file = File::open("config.yml").expect("Config file not found");
    let reader = BufReader::new(file);
    let config: Config = serde_yml::from_reader(reader).expect("Error reading config");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    println!("{}", dev_info());
    
    let mut client =
        Client::builder(config.token, intents).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}