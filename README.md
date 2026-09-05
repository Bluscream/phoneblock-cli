# phoneblock-cli

[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://unlicense.org/)
[![Language: Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)

A high-performance command-line interface (CLI) for [PhoneBlock](https://phoneblock.net), built in Rust.

PhoneBlock is the community-driven spam call blocker for FRITZ!Box routers and mobile devices. `phoneblock-cli` allows you to manage your personal blacklists, whitelists, wildcard blocks, rate caller numbers, check SPAM ratings, and monitor real-time PhoneBlock community statistics directly from your terminal.

---

## Features

- 🔐 **Authentication**: Fast web-based and token login (`phoneblock login`).
- 🚫 **Personal Blacklist Management**: Add, list, comment on, and remove blocked numbers (`phoneblock blacklist`).
- ✳️ **Wildcard Prefix Blocking**: Auto-detects wildcard asterisks (e.g. `phoneblock blacklist add "+49301234*"`) to add or remove wildcard pattern blocks (`+49301234*`).
- ⚪ **Personal Whitelist Management**: Manage trusted numbers (`phoneblock whitelist`).
- 📊 **Status & Live Community Metrics**: View account settings, active database size, top current spammers, and hourly spam report feeds (`phoneblock status`).
- 🔎 **Lookup & Search**: Perform privacy-preserving k-anonymity SHA1 prefix lookups (`phoneblock check --privacy`) or full search breakdowns (`phoneblock search`).
- 🔄 **Blocklist Synchronization**: Download and export the community SPAM blocklist to JSON (`phoneblock sync`).

---

## Installation

### Pre-built Binaries

Download the latest release binaries for Linux (`phoneblock-linux-amd64`) or Windows (`phoneblock.exe`) from the [Releases](https://github.com/Bluscream/phoneblock-cli/releases) page.

Move the binary into your PATH (e.g., `~/.local/bin/phoneblock` on Linux or `%PATH%` on Windows).

### Building from Source

Requirements:
- [Rust & Cargo](https://rustup.rs/) (1.75+)

```bash
git clone https://github.com/Bluscream/phoneblock-cli.git
cd phoneblock-cli
cargo build --release
```

The compiled binary will be placed at `target/release/phoneblock`.

---

## Usage Guide

### 1. Login
```bash
phoneblock login --user <USERNAME_OR_ID>
```

### 2. Status & Community Metrics
```bash
phoneblock status
```

### 3. Check / Search a Phone Number
```bash
# Quick lookup
phoneblock check "+4917650642602"

# Privacy-preserving k-anonymity SHA1 lookup
phoneblock check "+4917650642602" --privacy

# Detailed search breakdown with community votes and comments
phoneblock search "+4917650642602"
```

### 4. Manage Blacklist & Wildcard Blocks
```bash
# List all blacklisted numbers
phoneblock blacklist list

# Add a specific number
phoneblock blacklist add "+4940299961703" --comment "Advertising caller"

# Add a wildcard prefix block (blocks all numbers starting with +49301234)
phoneblock blacklist add "+49301234*"

# Remove a number or wildcard block
phoneblock blacklist remove "+4940299961703"
phoneblock blacklist remove "+49301234*"
```

### 5. Manage Whitelist
```bash
phoneblock whitelist list
phoneblock whitelist add "+4915112345678" --comment "Family number"
phoneblock whitelist remove "+4915112345678"
```

### 6. Rate Numbers
```bash
phoneblock rate "+4917650642602" --rating E_ADVERTISING --comment "Telemarketing scam"
```

### 7. Sync Community Blocklist
```bash
phoneblock sync --limit 100 --output blocklist.json
```

---

## License

This project is released into the public domain under the [Unlicense](LICENSE).
