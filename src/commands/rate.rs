use anyhow::{anyhow, Result};
use colored::Colorize;

use crate::api::PhoneBlockClient;
use crate::config::Config;

pub async fn handle_rate(phone: String, rating: String, comment: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    let normalized_rating = match rating.to_uppercase().as_str() {
        "A" | "LEGITIMATE" | "A_LEGITIMATE" => "A_LEGITIMATE",
        "B" | "MISSED" | "B_MISSED" => "B_MISSED",
        "C" | "PING" | "C_PING" => "C_PING",
        "D" | "POLL" | "SURVEY" | "D_POLL" => "D_POLL",
        "E" | "ADVERTISING" | "SPAM" | "E_ADVERTISING" => "E_ADVERTISING",
        "F" | "GAMBLE" | "F_GAMBLE" => "F_GAMBLE",
        "G" | "FRAUD" | "SCAM" | "G_FRAUD" => "G_FRAUD",
        other => {
            return Err(anyhow!(
                "Invalid rating '{}'. Valid ratings are: A_LEGITIMATE, B_MISSED, C_PING, D_POLL, E_ADVERTISING, F_GAMBLE, G_FRAUD",
                other
            ));
        }
    };

    println!(
        "Submitting rating {} for number {}...",
        normalized_rating.bold().yellow(),
        phone.bold().cyan()
    );

    client.rate_number(&phone, normalized_rating, comment.as_deref()).await?;
    println!("{} Successfully submitted rating for {}!", "✓".green().bold(), phone.cyan());

    Ok(())
}
