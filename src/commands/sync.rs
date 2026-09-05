use anyhow::Result;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;
use std::fs;

use crate::api::PhoneBlockClient;
use crate::config::Config;

pub async fn handle_sync(
    limit: Option<usize>,
    since: Option<i64>,
    output: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    println!("Fetching PhoneBlock community blocklist...");
    let blocklist = client.get_blocklist(since, limit).await?;

    println!(
        "{} Downloaded blocklist version {} ({} entries)",
        "✓".green().bold(),
        blocklist.version.unwrap_or(0).to_string().cyan(),
        blocklist.numbers.len().to_string().bold()
    );

    if let Some(path) = output {
        let json_data = serde_json::to_string_pretty(&blocklist)?;
        fs::write(&path, json_data)?;
        println!("Saved blocklist to {}", path.bold().cyan());
    } else {
        let mut table = Table::new();
        table.load_preset(UTF8_FULL);
        table.set_header(vec!["Phone", "Rating", "Votes"]);

        let display_limit = limit.unwrap_or(15).min(blocklist.numbers.len());
        for num in blocklist.numbers.iter().take(display_limit) {
            table.add_row(vec![
                num.phone.bold().red().to_string(),
                num.rating.clone().unwrap_or_default().yellow().to_string(),
                num.votes.unwrap_or(0).to_string(),
            ]);
        }

        println!("{}", table);

        if blocklist.numbers.len() > display_limit {
            println!(
                " (Showing first {} entries out of {}. Use --output <file> to export full list)",
                display_limit,
                blocklist.numbers.len()
            );
        }
    }

    Ok(())
}
