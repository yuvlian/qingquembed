use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;
use regex::Regex;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

struct Handler;

#[derive(Deserialize)]
struct Config {
    token: String,
}

const TNKTOK: &str = r"https://tnktok.com/"; // TIKTOK.COM
const DDINSTAGRAM: &str = r"https://ddinstagram.com/"; // INSTAGRAM.COM
const VXTWITTER: &str = r"https://vxtwitter.com/"; // TWITTER.COM
const FIXVX: &str = r"https://fixvx.com/"; // X.COM

lazy_static! {
    static ref TIKTOK_REGEX: Regex = Regex::new(r"https?://(www\.)?(tiktok\.com|vt\.tiktok\.com|vm\.tiktok\.com)/?").unwrap();
    static ref INSTAGRAM_REGEX: Regex = Regex::new(r"https?://(www\.)?(instagram\.com|instagr\.am)/?").unwrap();
    static ref TWITTER_REGEX: Regex = Regex::new(r"https?://(www\.)?(twitter\.com)/?").unwrap();
    static ref X_REGEX: Regex = Regex::new(r"https?://(www\.)?(x\.com)/?").unwrap();
}

#[async_trait]
impl EventHandler for Handler {

    async fn message(&self, ctx: Context, msg: Message) {
        
        // BOT DONT LOOP!!!
        if msg.author.bot {
            return;
        }

        // MATCH THE STUFF YEA
        if TIKTOK_REGEX.is_match(&msg.content) || INSTAGRAM_REGEX.is_match(&msg.content) || TWITTER_REGEX.is_match(&msg.content) || X_REGEX.is_match(&msg.content) {
            let tiktok_replace = TIKTOK_REGEX.replace_all(&msg.content, TNKTOK);
            let instagram_replace = INSTAGRAM_REGEX.replace_all(&tiktok_replace, DDINSTAGRAM);
            let twitter_replace = TWITTER_REGEX.replace_all(&instagram_replace, VXTWITTER);
            let x_replace = X_REGEX.replace_all(&twitter_replace, FIXVX);
            
            // ADD AUTHOR
            let replaced_content = format!("[<@{}>] {}", msg.author.id, x_replace);
            
            // SEND MODIFIED MESSAGE
            if let Err(why) = msg.channel_id.say(&ctx.http, replaced_content).await {
                println!("Error sending message: {:?}", why);
            }
            
            // DELETE ORIGINAL MESSAGE
            if let Err(why) = msg.delete(&ctx.http).await {
                println!("Error deleting message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("autism embed fixer bot thing made by https://github.com/yuvlian");
    let file = File::open("config.yml").expect("Config file not found");
    let reader = BufReader::new(file);
    let config: Config = serde_yml::from_reader(reader).expect("Error reading config");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&config.token, intents).event_handler(Handler).await.expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

}