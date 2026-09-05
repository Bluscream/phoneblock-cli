#![allow(dead_code)]

use anyhow::{anyhow, Result};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::{Client, StatusCode};
use std::collections::HashMap;

use crate::api::models::*;
use crate::config::Config;

pub struct PhoneBlockClient {
    client: Client,
    base_url: String,
    config: Config,
}

impl PhoneBlockClient {
    pub fn new(config: &Config) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("phoneblock-cli/1.0 (Rust)"),
        );

        if let Some(token) = &config.token {
            let auth_val = format!("Bearer {}", token);
            if let Ok(hv) = HeaderValue::from_str(&auth_val) {
                headers.insert(AUTHORIZATION, hv);
            }
        } else if let (Some(u), Some(p)) = (&config.username, &config.password) {
            let auth_val = format!(
                "Basic {}",
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, format!("{}:{}", u, p))
            );
            if let Ok(hv) = HeaderValue::from_str(&auth_val) {
                headers.insert(AUTHORIZATION, hv);
            }
        }

        let client = Client::builder()
            .default_headers(headers)
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let base_url = config.api_url.trim_end_matches('/').to_string();

        Ok(Self {
            client,
            base_url,
            config: config.clone(),
        })
    }

    pub async fn login_web(username: &str, password: &str) -> Result<String> {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let mut params = HashMap::new();
        params.insert("userName", username);
        params.insert("password", password);
        params.insert("rememberMe", "true");

        let res = client
            .post("https://phoneblock.net/phoneblock/check-login")
            .form(&params)
            .send()
            .await?;

        let mut token = None;
        for cookie in res.headers().get_all(reqwest::header::SET_COOKIE) {
            if let Ok(cookie_str) = cookie.to_str() {
                for part in cookie_str.split(';') {
                    let part = part.trim();
                    if part.starts_with("pb-login=") {
                        token = Some(part.trim_start_matches("pb-login=").to_string());
                        break;
                    }
                }
            }
        }

        if let Some(t) = token {
            Ok(t)
        } else if res.status().is_redirection() {
            Err(anyhow!("Login redirect received, but no pb-login token cookie was set. Please check credentials."))
        } else {
            let status = res.status();
            let body = res.text().await.unwrap_or_default();
            Err(anyhow!("Login failed with HTTP status {}: {}", status, body))
        }
    }

    pub async fn test_connection(&self) -> Result<String> {
        let url = format!("{}/test", self.base_url);
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let text = res.text().await?;
            Ok(text)
        } else {
            Err(anyhow!("Authentication failed (HTTP {})", status))
        }
    }

    pub async fn get_account(&self) -> Result<AccountSettings> {
        let url = format!("{}/account", self.base_url);
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let account: AccountSettings = res.json().await?;
            Ok(account)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to fetch account: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn update_account(&self, req: &UpdateAccountRequest) -> Result<AccountSettings> {
        let url = format!("{}/account", self.base_url);
        let res = self.client.put(&url).json(req).send().await?;
        let status = res.status();
        if status.is_success() {
            let account: AccountSettings = res.json().await?;
            Ok(account)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to update account: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn get_blacklist(&self) -> Result<NumberList> {
        let url = format!("{}/blacklist", self.base_url);
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let list: NumberList = res.json().await?;
            Ok(list)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to fetch blacklist: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn delete_blacklist(&self, phone: &str) -> Result<()> {
        let url = format!("{}/blacklist/{}", self.base_url, urlencoding::encode(phone));
        let res = self.client.delete(&url).send().await?;
        let status = res.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to delete from blacklist: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn update_blacklist_comment(&self, phone: &str, comment: &str) -> Result<()> {
        let url = format!("{}/blacklist/{}", self.base_url, urlencoding::encode(phone));
        let body = serde_json::json!({ "phone": phone, "comment": comment });
        let res = self.client.put(&url).json(&body).send().await?;
        let status = res.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to update blacklist comment: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn get_whitelist(&self) -> Result<NumberList> {
        let url = format!("{}/whitelist", self.base_url);
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let list: NumberList = res.json().await?;
            Ok(list)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to fetch whitelist: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn delete_whitelist(&self, phone: &str) -> Result<()> {
        let url = format!("{}/whitelist/{}", self.base_url, urlencoding::encode(phone));
        let res = self.client.delete(&url).send().await?;
        let status = res.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to delete from whitelist: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn update_whitelist_comment(&self, phone: &str, comment: &str) -> Result<()> {
        let url = format!("{}/whitelist/{}", self.base_url, urlencoding::encode(phone));
        let body = serde_json::json!({ "phone": phone, "comment": comment });
        let res = self.client.put(&url).json(&body).send().await?;
        let status = res.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to update whitelist comment: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn check_number(&self, phone: &str) -> Result<PhoneInfo> {
        let url = format!("{}/num/{}", self.base_url, urlencoding::encode(phone));
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let info: PhoneInfo = res.json().await?;
            Ok(info)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to check number: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn check_hash(&self, sha1: &str) -> Result<PhoneInfo> {
        let url = format!("{}/check?sha1={}", self.base_url, sha1);
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let info: PhoneInfo = res.json().await?;
            Ok(info)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to check hash: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn check_prefix(&self, sha1_prefix: &str) -> Result<PrefixCheckResult> {
        let url = format!("{}/check-prefix?sha1={}", self.base_url, sha1_prefix);
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let result: PrefixCheckResult = res.json().await?;
            Ok(result)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to check prefix: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn search_number(&self, phone: &str) -> Result<SearchResult> {
        let url = format!("{}/search/{}", self.base_url, urlencoding::encode(phone));
        let res = self.client.get(&url).send().await?;
        let status = res.status();
        if status.is_success() {
            let search: SearchResult = res.json().await?;
            Ok(search)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to search number: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn rate_number(&self, phone: &str, rating: &str, comment: Option<&str>) -> Result<()> {
        let url = format!("{}/rate", self.base_url);
        let body = RateRequest {
            phone: phone.to_string(),
            rating: rating.to_string(),
            comment: comment.map(|s| s.to_string()),
        };
        let res = self.client.post(&url).json(&body).send().await?;
        let status = res.status();
        if status.is_success() {
            Ok(())
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to rate number: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn report_call(&self, phone: &str) -> Result<()> {
        let url = format!("{}/report-call/{}", self.base_url, urlencoding::encode(phone));
        let res = self.client.post(&url).send().await?;
        let status = res.status();
        if status.is_success() || status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to report call: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn get_blocklist(
        &self,
        since: Option<i64>,
        limit: Option<usize>,
    ) -> Result<BlocklistResponse> {
        let mut url = format!("{}/blocklist?format=json", self.base_url);
        if let Some(s) = since {
            url.push_str(&format!("&since={}", s));
        }
        if let Some(l) = limit {
            url.push_str(&format!("&limit={}", l));
        }

        let mut req_builder = self.client.get(&url);

        // Fallback to Basic Auth if credentials exist in config (for endpoints like /blocklist)
        if let (Some(u), Some(p)) = (&self.config.username, &self.config.password) {
            req_builder = req_builder.basic_auth(u, Some(p));
        }

        let res = req_builder.send().await?;
        let status = res.status();
        if status.is_success() {
            let blocklist: BlocklistResponse = res.json().await?;
            Ok(blocklist)
        } else {
            let err_text = res.text().await.unwrap_or_default();
            Err(anyhow!("Failed to fetch blocklist: HTTP {} - {}", status, err_text))
        }
    }

    pub async fn add_wildcard(&self, prefix: &str) -> Result<()> {
        let url = "https://phoneblock.net/phoneblock/update-settings?action=lists";
        let mut params = HashMap::new();
        params.insert("add-wc", prefix);
        params.insert("redirect", "/blacklist");

        let mut req_builder = self.client.post(url).form(&params);
        if let (Some(u), Some(p)) = (&self.config.username, &self.config.password) {
            req_builder = req_builder.basic_auth(u, Some(p));
        }
        if let Some(token) = &self.config.token {
            req_builder = req_builder.header(
                reqwest::header::COOKIE,
                format!("pb-login={}", token),
            );
        }
        let res = req_builder.send().await?;
        if res.status().is_success() || res.status().is_redirection() {
            Ok(())
        } else {
            Err(anyhow!("Failed to add wildcard block: HTTP {}", res.status()))
        }
    }

    pub async fn get_wildcards(&self) -> Result<Vec<String>> {
        let url = "https://phoneblock.net/phoneblock/blacklist";
        let mut req_builder = self.client.get(url);
        if let (Some(u), Some(p)) = (&self.config.username, &self.config.password) {
            req_builder = req_builder.basic_auth(u, Some(p));
        }
        if let Some(token) = &self.config.token {
            req_builder = req_builder.header(
                reqwest::header::COOKIE,
                format!("pb-login={}", token),
            );
        }
        let res = req_builder.send().await?;
        let html = res.text().await?;

        let mut wildcards = Vec::new();
        let re = regex::Regex::new(r#"name="(wc-[^"]+)""#)?;
        for cap in re.captures_iter(&html) {
            if let Some(m) = cap.get(1) {
                wildcards.push(m.as_str().to_string());
            }
        }
        Ok(wildcards)
    }

    pub async fn delete_wildcard(&self, prefix: &str) -> Result<()> {
        let url = "https://phoneblock.net/phoneblock/update-settings?action=lists";
        let bl_html = {
            let mut req = self.client.get("https://phoneblock.net/phoneblock/blacklist");
            if let (Some(u), Some(p)) = (&self.config.username, &self.config.password) {
                req = req.basic_auth(u, Some(p));
            }
            if let Some(token) = &self.config.token {
                req = req.header(reqwest::header::COOKIE, format!("pb-login={}", token));
            }
            req.send().await?.text().await?
        };

        let digits: String = prefix.chars().filter(|c| c.is_ascii_digit()).collect();
        let national_digits = if digits.starts_with("49") {
            format!("0{}", &digits[2..])
        } else {
            digits.clone()
        };

        let re = regex::Regex::new(r#"name="(wc-[^"]+)""#)?;
        let mut target_name = None;
        for cap in re.captures_iter(&bl_html) {
            if let Some(m) = cap.get(1) {
                let field = m.as_str();
                let field_digits: String = field.chars().filter(|c| c.is_ascii_digit()).collect();
                if field_digits == digits
                    || field_digits == national_digits
                    || digits.contains(&field_digits)
                    || national_digits.contains(&field_digits)
                    || (!field_digits.is_empty() && (digits.ends_with(&field_digits) || national_digits.ends_with(&field_digits)))
                {
                    target_name = Some(field.to_string());
                    break;
                }
            }
        }

        let field_name = target_name.ok_or_else(|| anyhow!("Wildcard block for prefix '{}' not found", prefix))?;

        let mut params = HashMap::new();
        params.insert(field_name.as_str(), "on");
        params.insert("redirect", "/blacklist");

        let mut req_builder = self.client.post(url).form(&params);
        if let (Some(u), Some(p)) = (&self.config.username, &self.config.password) {
            req_builder = req_builder.basic_auth(u, Some(p));
        }
        if let Some(token) = &self.config.token {
            req_builder = req_builder.header(
                reqwest::header::COOKIE,
                format!("pb-login={}", token),
            );
        }

        let res = req_builder.send().await?;
        if res.status().is_success() || res.status().is_redirection() {
            Ok(())
        } else {
            Err(anyhow!("Failed to delete wildcard block: HTTP {}", res.status()))
        }
    }

    pub async fn get_community_status(&self) -> Result<CommunityStatus> {
        let url = "https://phoneblock.net/phoneblock/status";
        let res = self.client.get(url).send().await?;
        let html = res.text().await?;

        let active_count = regex::Regex::new(r"(\d+)\s+active numbers")?
            .captures(&html)
            .and_then(|cap| cap.get(1))
            .and_then(|m| m.as_str().parse::<usize>().ok());

        let parse_table = |keyword: &str| -> Vec<CommunitySpamEntry> {
            let mut entries = Vec::new();
            if let Some(idx) = html.find(keyword) {
                let sub = &html[idx..];
                let re_tr = regex::Regex::new(r"(?s)<tr>(.*?)</tr>").unwrap();
                let re_td = regex::Regex::new(r"(?s)<t[dh][^>]*>(.*?)</t[dh]>").unwrap();
                let re_strip = regex::Regex::new(r"<[^>]+>").unwrap();

                for cap in re_tr.captures_iter(sub).skip(1).take(5) {
                    let tr_html = &cap[1];
                    let cols: Vec<String> = re_td
                        .captures_iter(tr_html)
                        .map(|c| re_strip.replace_all(&c[1], "").trim().to_string())
                        .collect();
                    if cols.len() >= 4 {
                        entries.push(CommunitySpamEntry {
                            phone: cols[0].clone(),
                            val1: cols[1].clone(),
                            val2: cols[2].clone(),
                            last_activity: cols[3].clone(),
                        });
                    }
                }
            }
            entries
        };

        Ok(CommunityStatus {
            active_blocklist_count: active_count,
            recent_searches: parse_table("Current search queries"),
            recent_reports: parse_table("Spam reports of the last hour"),
            newest_blocklist: parse_table("Newest entries in the block list"),
            top_spammers: parse_table("Current top spammers"),
            most_searched: parse_table("Most searched numbers"),
        })
    }
}

