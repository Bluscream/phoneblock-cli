#![allow(clippy::module_name_repetitions)]
#![warn(clippy::too_many_lines, clippy::all)]

mod api;
mod commands;
mod config;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "phoneblock",
    author = "PhoneBlock CLI Team",
    version = "0.1.0",
    about = "A command-line interface for PhoneBlock (https://phoneblock.net)",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Log in to PhoneBlock and save credentials/session token
    Login {
        /// PhoneBlock Username / User ID
        #[arg(short, long)]
        user: String,

        /// PhoneBlock Password (will be prompted if omitted)
        #[arg(short, long)]
        password: Option<String>,
    },

    /// Log out and clear saved credentials
    Logout,

    /// Check connection health and display user account status
    Status,

    /// Check whether a phone number is on the SPAM blocklist
    Check {
        /// Phone number to check (e.g. +49123456789)
        phone: String,

        /// Use privacy-preserving k-anonymity SHA1 prefix lookup
        #[arg(short, long)]
        privacy: bool,
    },

    /// Search detailed information, ratings breakdown, and user comments for a number
    Search {
        /// Phone number to search
        phone: String,
    },

    /// Manage your personal blacklist (blocked callers)
    Blacklist {
        #[command(subcommand)]
        action: BlacklistAction,
    },

    /// Manage your personal whitelist (allowed callers)
    Whitelist {
        #[command(subcommand)]
        action: WhitelistAction,
    },

    /// Submit a rating and comment for a phone number
    Rate {
        /// Phone number to rate (e.g. +49123456789)
        phone: String,

        /// Rating type: A_LEGITIMATE, B_MISSED, C_PING, D_POLL, E_ADVERTISING, F_GAMBLE, G_FRAUD
        #[arg(short, long)]
        rating: String,

        /// Optional comment explaining the rating
        #[arg(short, long)]
        comment: Option<String>,
    },

    /// Synchronize / download the community SPAM blocklist
    Sync {
        /// Limit maximum entries returned
        #[arg(short, long)]
        limit: Option<usize>,

        /// Fetch changes since specific blocklist version (incremental sync)
        #[arg(short, long)]
        since: Option<i64>,

        /// Export blocklist to JSON file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
enum BlacklistAction {
    /// List all numbers on your personal blacklist
    List,

    /// Add a number to your personal blacklist
    Add {
        phone: String,
        #[arg(short, long)]
        comment: Option<String>,
    },

    /// Remove a number from your personal blacklist
    Remove { phone: String },

    /// Update comment for a blacklisted number
    Comment { phone: String, comment: String },
}

#[derive(Subcommand)]
enum WhitelistAction {
    /// List all numbers on your personal whitelist
    List,

    /// Add a number to your personal whitelist
    Add {
        phone: String,
        #[arg(short, long)]
        comment: Option<String>,
    },

    /// Remove a number from your personal whitelist
    Remove { phone: String },

    /// Update comment for a whitelisted number
    Comment { phone: String, comment: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Login { user, password } => {
            commands::auth::handle_login(user, password).await?;
        }
        Commands::Logout => {
            commands::auth::handle_logout().await?;
        }
        Commands::Status => {
            commands::status::handle_status().await?;
        }
        Commands::Check { phone, privacy } => {
            commands::check::handle_check(phone, privacy).await?;
        }
        Commands::Search { phone } => {
            commands::search::handle_search(phone).await?;
        }
        Commands::Blacklist { action } => match action {
            BlacklistAction::List => commands::blacklist::handle_blacklist_list().await?,
            BlacklistAction::Add { phone, comment } => {
                commands::blacklist::handle_blacklist_add(phone, comment).await?
            }
            BlacklistAction::Remove { phone } => {
                commands::blacklist::handle_blacklist_remove(phone).await?
            }
            BlacklistAction::Comment { phone, comment } => {
                commands::blacklist::handle_blacklist_comment(phone, comment).await?
            }
        },
        Commands::Whitelist { action } => match action {
            WhitelistAction::List => commands::whitelist::handle_whitelist_list().await?,
            WhitelistAction::Add { phone, comment } => {
                commands::whitelist::handle_whitelist_add(phone, comment).await?
            }
            WhitelistAction::Remove { phone } => {
                commands::whitelist::handle_whitelist_remove(phone).await?
            }
            WhitelistAction::Comment { phone, comment } => {
                commands::whitelist::handle_whitelist_comment(phone, comment).await?
            }
        },
        Commands::Rate {
            phone,
            rating,
            comment,
        } => {
            commands::rate::handle_rate(phone, rating, comment).await?;
        }
        Commands::Sync {
            limit,
            since,
            output,
        } => {
            commands::sync::handle_sync(limit, since, output).await?;
        }
    }

    Ok(())
}
