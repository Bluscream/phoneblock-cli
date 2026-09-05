use anyhow::Result;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

use crate::api::PhoneBlockClient;
use crate::config::Config;

pub async fn handle_blacklist_list() -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    let list = client.get_blacklist().await?;
    println!("Personal Blacklist ({} entries):", list.numbers.len().to_string().bold());

    if list.numbers.is_empty() {
        println!("Your personal blacklist is empty.");
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Phone", "Display Label", "Rating", "Comment"]);

    for item in list.numbers {
        table.add_row(vec![
            item.phone.bold().red().to_string(),
            item.label.unwrap_or_default(),
            item.rating.unwrap_or_else(|| "BLOCKED".to_string()).yellow().to_string(),
            item.comment.unwrap_or_default(),
        ]);
    }

    println!("{}", table);
    Ok(())
}

pub async fn handle_blacklist_add(phone: String, comment: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    if phone.contains('*') || phone.contains('%') {
        let prefix = phone.trim_end_matches('*').trim_end_matches('%');
        println!("Adding wildcard block prefix {}...", prefix.bold().red());
        client.add_wildcard(prefix).await?;
        println!("{} Successfully added wildcard block {}* to PhoneBlock!", "✓".green().bold(), prefix.cyan());
        return Ok(());
    }

    println!("Adding {} to personal blacklist...", phone.bold().red());
    // Rating as B_MISSED or E_ADVERTISING adds to personal blacklist
    client.rate_number(&phone, "B_MISSED", comment.as_deref()).await?;
    println!("{} Successfully added {} to personal blacklist!", "✓".green().bold(), phone.cyan());
    Ok(())
}

pub async fn handle_blacklist_remove(phone: String) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    if phone.contains('*') || phone.contains('%') {
        let prefix = phone.trim_end_matches('*').trim_end_matches('%');
        println!("Removing wildcard block prefix {}...", prefix.bold().red());
        client.delete_wildcard(prefix).await?;
        println!("{} Successfully removed wildcard block {}* from PhoneBlock!", "✓".green().bold(), prefix.cyan());
        return Ok(());
    }

    println!("Removing {} from personal blacklist...", phone.bold());
    client.delete_blacklist(&phone).await?;
    println!("{} Successfully removed {} from blacklist!", "✓".green().bold(), phone.cyan());
    Ok(())
}

pub async fn handle_blacklist_comment(phone: String, comment: String) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    client.update_blacklist_comment(&phone, &comment).await?;
    println!("{} Updated comment for {}: {}", "✓".green().bold(), phone.cyan(), comment.italic());
    Ok(())
}
