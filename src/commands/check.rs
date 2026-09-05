use anyhow::Result;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use sha1::{Digest, Sha1};

use crate::api::models::PhoneInfo;
use crate::api::PhoneBlockClient;
use crate::config::Config;

pub async fn handle_check(phone: String, privacy: bool) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    if privacy {
        println!("Performing privacy-preserving k-anonymity lookup...");
        let mut hasher = Sha1::new();
        hasher.update(phone.as_bytes());
        let hash_hex = hex::encode(hasher.finalize());
        let prefix = &hash_hex[..4];

        let result = client.check_prefix(prefix).await?;
        println!("Found {} candidate numbers for hash prefix '{}'", result.numbers.len(), prefix);

        let match_found = result.numbers.iter().find(|n| {
            let mut h = Sha1::new();
            h.update(n.phone.as_bytes());
            hex::encode(h.finalize()) == hash_hex
        });

        if let Some(info) = match_found {
            print_phone_info(info);
        } else {
            println!("{} Number {} was not found in blocklist (candidate bucket clean).", "✓".green().bold(), phone.cyan());
        }
    } else {
        println!("Checking phone number {}...", phone.bold().cyan());
        let info = client.check_number(&phone).await?;
        print_phone_info(&info);
    }

    Ok(())
}

pub fn print_phone_info(info: &PhoneInfo) {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Field", "Details"]);

    table.add_row(vec!["Phone", info.phone.as_str()]);

    let lbl = info.label.as_deref().unwrap_or("");
    if !lbl.is_empty() {
        table.add_row(vec!["Display Label", lbl]);
    }

    let loc = info.location.as_deref().unwrap_or("");
    if !loc.is_empty() {
        table.add_row(vec!["Location / Carrier", loc]);
    }

    let rating_colored = match info.rating.as_deref().unwrap_or("UNKNOWN") {
        "G_FRAUD" => "G_FRAUD (Fraud / Scam)".red().bold().to_string(),
        "E_ADVERTISING" => "E_ADVERTISING (Advertising)".yellow().bold().to_string(),
        "F_GAMBLE" => "F_GAMBLE (Gambling)".magenta().bold().to_string(),
        "C_PING" => "C_PING (Ping / Ping Call)".yellow().to_string(),
        "D_POLL" => "D_POLL (Survey / Polling)".yellow().to_string(),
        "B_MISSED" => "B_MISSED (Missed Call)".blue().to_string(),
        "A_LEGITIMATE" => "A_LEGITIMATE (Legitimate)".green().bold().to_string(),
        other => other.to_string(),
    };
    table.add_row(vec!["Rating", &rating_colored]);

    let votes_str = info.votes.unwrap_or(0).to_string();
    table.add_row(vec!["Spam Votes", &votes_str]);

    let wildcard_str = info.votes_wildcard.unwrap_or(0).to_string();
    table.add_row(vec!["Wildcard Votes", &wildcard_str]);

    if let Some(conf) = info.spam_confidence {
        let conf_str = format!("{}%", conf);
        let colored_conf = if conf > 75 {
            conf_str.red().bold().to_string()
        } else if conf > 40 {
            conf_str.yellow().to_string()
        } else {
            conf_str.green().to_string()
        };
        table.add_row(vec!["Spam Confidence", &colored_conf]);
    }

    if let Some(calls) = info.calls {
        let calls_str = calls.to_string();
        table.add_row(vec!["Intercepted Calls", &calls_str]);
    }

    let is_blacklisted = if info.black_listed.unwrap_or(false) {
        "YES".red().bold().to_string()
    } else {
        "No".to_string()
    };
    table.add_row(vec!["On Personal Blacklist", &is_blacklisted]);

    let is_whitelisted = if info.white_listed.unwrap_or(false) {
        "YES".green().bold().to_string()
    } else {
        "No".to_string()
    };
    table.add_row(vec!["On Personal Whitelist", &is_whitelisted]);

    if let Some(comment) = &info.user_comment {
        let c_str = comment.cyan().to_string();
        table.add_row(vec!["Your Saved Note", &c_str]);
    }

    println!("\n{}", table);
}
