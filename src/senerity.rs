use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!rust" {
            if let Err(why) = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content("Join Rust's Discord: https://discord.gg/rust-lang")
                })
                .await
            {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = "YOUR_BOT_TOKEN_HERE";

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
