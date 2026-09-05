use anyhow::Result;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

use crate::api::PhoneBlockClient;
use crate::config::Config;

pub async fn handle_whitelist_list() -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    let list = client.get_whitelist().await?;
    println!("Personal Whitelist ({} entries):", list.numbers.len().to_string().bold());

    if list.numbers.is_empty() {
        println!("Your personal whitelist is empty.");
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_header(vec!["Phone", "Display Label", "Rating", "Comment"]);

    for item in list.numbers {
        table.add_row(vec![
            item.phone.bold().green().to_string(),
            item.label.unwrap_or_default(),
            item.rating.unwrap_or_else(|| "ALLOWED".to_string()).green().to_string(),
            item.comment.unwrap_or_default(),
        ]);
    }

    println!("{}", table);
    Ok(())
}

pub async fn handle_whitelist_add(phone: String, comment: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    println!("Adding {} to personal whitelist...", phone.bold().green());
    // Rating as A_LEGITIMATE adds to personal whitelist
    client.rate_number(&phone, "A_LEGITIMATE", comment.as_deref()).await?;
    println!("{} Successfully added {} to personal whitelist!", "✓".green().bold(), phone.cyan());
    Ok(())
}

pub async fn handle_whitelist_remove(phone: String) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    println!("Removing {} from personal whitelist...", phone.bold());
    client.delete_whitelist(&phone).await?;
    println!("{} Successfully removed {} from whitelist!", "✓".green().bold(), phone.cyan());
    Ok(())
}

pub async fn handle_whitelist_comment(phone: String, comment: String) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    client.update_whitelist_comment(&phone, &comment).await?;
    println!("{} Updated comment for {}: {}", "✓".green().bold(), phone.cyan(), comment.italic());
    Ok(())
}
