use reqwest::Client;
use serde_json::json;


async fn telegram(msg: &str) -> Result<(), reqwest::Error> {
    let token = std::env::var("TELEGRAM_TOKEN").expect("TELEGRAM_TOKEN not set");
    let group_id = std::env::var("TELEGRAM_CHAT_ID").expect("TELEGRAM_CHAT_ID not set");

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    Client::new()
        .post(url)
        .json(&json!({
            "chat_id": group_id,
            "text": msg
        }))
        .send()
        .await?;

    Ok(())
}

async fn slack(msg: &str) -> Result<(), reqwest::Error> {
    let webhook = std::env::var("SLACK_WEBHOOK").expect("SLACK_WEBHOOK not set");

    Client::new()
        .post(webhook)
        .json(&json!({
            "text": msg
        }))
        .send()
        .await?;

    Ok(())
}


#[tokio::main]
async fn main() {
    let msg = "⚚ *ICON TRADING*\n Sharpe ratio over 3000!";
    
    match telegram(msg).await {
        Ok(_) => println!("Telegram message sent successfully"),
        Err(e) => eprintln!("Error sending Telegram message: {}", e),
    }
    
    match slack(msg).await {
        Ok(_) => println!("Slack message sent successfully"),
        Err(e) => eprintln!("Error sending Slack message: {}", e),
    }
}




