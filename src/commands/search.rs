use anyhow::Result;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

use crate::api::PhoneBlockClient;
use crate::commands::check::print_phone_info;
use crate::config::Config;

pub async fn handle_search(phone: String) -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    println!("Searching detailed information for {}...", phone.bold().cyan());
    let result = client.search_number(&phone).await?;

    if let Some(info) = &result.info {
        print_phone_info(info);
    }

    if let Some(ai) = &result.ai_summary {
        println!("\n🤖 {}", "AI Summary:".bold().yellow());
        println!("  {}", ai);
    }

    if let Some(ratings) = &result.ratings {
        if !ratings.is_empty() {
            println!("\n📊 {}", "Rating Breakdown:".bold());
            let mut r_table = Table::new();
            r_table.load_preset(UTF8_FULL);
            r_table.set_header(vec!["Rating Type", "Vote Count"]);
            for entry in ratings {
                r_table.add_row(vec![entry.key.as_str(), &entry.value.to_string()]);
            }
            println!("{}", r_table);
        }
    }

    if let Some(comments) = &result.comments {
        if !comments.is_empty() {
            println!("\n💬 {}", "Community Comments:".bold());
            for (idx, c) in comments.iter().enumerate() {
                println!(
                    "  {}. [{}] {}",
                    idx + 1,
                    c.rating.as_deref().unwrap_or("NOTE").yellow(),
                    c.comment.as_deref().unwrap_or("")
                );
            }
        }
    }

    if let Some(related) = &result.related_numbers {
        if !related.is_empty() {
            println!("\n🔗 {}", "Related Spam Numbers in Range:".bold());
            println!("  {}", related.join(", ").cyan());
        }
    }

    Ok(())
}
