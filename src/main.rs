use dialoguer::{Input, Confirm};
use reqwest::Client;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use serde_json::json;
use colored::*; // Dodaj ovu liniju

async fn send_message(webhook_url: &str, message: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let payload = json!({ "content": message });

    client.post(webhook_url)
        .json(&payload)
        .send()
        .await?;

    Ok(())
}

async fn start_discord_spam(webhook_url: &str, message: &str, interval: Duration) {
    let mut next_send_time = Instant::now();

    loop {
        let now = Instant::now();
        if now >= next_send_time {
            if let Err(e) = send_message(webhook_url, message).await {
                eprintln!("{}", format!("Error sending message: {}", e).red());
            }
            next_send_time = now + interval;
        }
        sleep(Duration::from_millis(100)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("{}", "🦠 | Disspam CLI! ( Ctrl + C = ⛔)".bold().cyan());

    let webhook_url: String = Input::new()
        .with_prompt("Discord webhook URL")
        .interact_text()
        .expect("Error entering URL");

    let message: String = Input::new()
        .with_prompt("💬?")
        .interact_text()
        .expect("Error entering message");

    let interval_str: String = Input::new()
        .with_prompt("⌛?")
        .interact_text()
        .expect("Error entering interval");

    let interval = interval_str
        .parse::<u64>()
        .expect("Invalid interval value");

    let interval = Duration::from_secs(interval);

    if Confirm::new()
        .with_prompt("🚀? (y/n)")
        .interact()
        .expect("Error confirming")
    {
        start_discord_spam(&webhook_url, &message, interval).await;
    } else {
        println!("{}", "Operation canceled.".red());
    }
}
