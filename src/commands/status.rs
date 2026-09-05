use anyhow::Result;
use colored::Colorize;
use comfy_table::presets::UTF8_FULL;
use comfy_table::Table;

use crate::api::PhoneBlockClient;
use crate::config::Config;

pub async fn handle_status() -> Result<()> {
    let config = Config::load()?;
    let client = PhoneBlockClient::new(&config)?;

    println!("{}", "Checking PhoneBlock API & Community status...".bold());

    match client.test_connection().await {
        Ok(msg) => {
            println!("{} API Connection: {}", "✓".green().bold(), msg.green());
        }
        Err(e) => {
            println!("{} API Connection: {}", "✗".red().bold(), e.to_string().red());
        }
    }

    match client.get_account().await {
        Ok(account) => {
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_header(vec!["Account Property", "Value"]);
            table.add_row(vec!["Display Name", &account.display_name.unwrap_or_default()]);
            table.add_row(vec!["User ID / Login", &account.login.unwrap_or_default()]);
            table.add_row(vec!["Email", &account.email.unwrap_or_default()]);
            table.add_row(vec!["Country Prefix", &account.dial_prefix.unwrap_or_default()]);
            table.add_row(vec!["Language", &account.lang.unwrap_or_default()]);

            println!("\n{}", table);
        }
        Err(e) => {
            println!("{} Account Info: {}", "✗".red().bold(), e.to_string().yellow());
            println!("Tip: Run 'phoneblock login' to set up authentication.");
        }
    }

    match client.get_community_status().await {
        Ok(status) => {
            println!("\n{}", "PhoneBlock Community Status (phoneblock.net/phoneblock/status)".cyan().bold());

            if let Some(count) = status.active_blocklist_count {
                println!("Active Blocklist Database Size: {} numbers", count.to_string().yellow().bold());
            }

            if !status.top_spammers.is_empty() {
                println!("\n{}", "Top Current Spammers:".bold().red());
                let mut table = Table::new();
                table.load_preset(UTF8_FULL);
                table.set_header(vec!["Phone Number", "Confidence", "Last Complaint", "Active Since"]);
                for entry in status.top_spammers.iter().take(5) {
                    table.add_row(vec![
                        entry.phone.bold().red().to_string(),
                        entry.val1.yellow().to_string(),
                        entry.val2.clone(),
                        entry.last_activity.clone(),
                    ]);
                }
                println!("{}", table);
            }

            if !status.recent_reports.is_empty() {
                println!("\n{}", "Recent Spam Reports (Last Hour):".bold().yellow());
                let mut table = Table::new();
                table.load_preset(UTF8_FULL);
                table.set_header(vec!["Phone Number", "Confidence", "Reported", "Active Since"]);
                for entry in status.recent_reports.iter().take(5) {
                    table.add_row(vec![
                        entry.phone.cyan().to_string(),
                        entry.val1.clone(),
                        entry.val2.clone(),
                        entry.last_activity.clone(),
                    ]);
                }
                println!("{}", table);
            }
        }
        Err(e) => {
            println!("{} Failed to fetch community status: {}", "✗".yellow().bold(), e);
        }
    }

    Ok(())
}
