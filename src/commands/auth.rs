use anyhow::Result;
use colored::Colorize;

use crate::api::PhoneBlockClient;
use crate::config::Config;

pub async fn handle_login(username: String, password: Option<String>) -> Result<()> {
    let mut config = Config::load()?;

    let pass = match password {
        Some(p) => p,
        None => {
            print!("Enter PhoneBlock password: ");
            use std::io::Write;
            std::io::stdout().flush()?;
            let mut p = String::new();
            std::io::stdin().read_line(&mut p)?;
            p.trim().to_string()
        }
    };

    println!("Authenticating with PhoneBlock as user {}...", username.bold());

    match PhoneBlockClient::login_web(&username, &pass).await {
        Ok(token) => {
            config.username = Some(username.clone());
            config.password = Some(pass);
            config.token = Some(token.clone());
            config.save()?;

            println!("{} Successfully logged in!", "✓".green().bold());
            println!("  Session token: {}", token.cyan());

            // Test connection
            let client = PhoneBlockClient::new(&config)?;
            if let Ok(account) = client.get_account().await {
                println!(
                    "  Connected account: {} ({})",
                    account.display_name.unwrap_or_default().bold(),
                    account.email.unwrap_or_default()
                );
            }
        }
        Err(err) => {
            println!(
                "{} Login failed: {}",
                "✗".red().bold(),
                err.to_string().red()
            );
        }
    }

    Ok(())
}

pub async fn handle_logout() -> Result<()> {
    let mut config = Config::load()?;
    config.token = None;
    config.password = None;
    config.save()?;

    println!("{} Successfully logged out. Credentials cleared.", "✓".green().bold());
    Ok(())
}
