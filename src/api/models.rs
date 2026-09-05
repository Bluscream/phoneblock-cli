#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSettings {
    pub lang: Option<String>,
    pub dial_prefix: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub login: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhoneInfo {
    pub phone: String,
    pub votes: Option<i64>,
    pub votes_wildcard: Option<i64>,
    pub rating: Option<String>,
    pub white_listed: Option<bool>,
    pub black_listed: Option<bool>,
    pub archived: Option<bool>,
    pub date_added: Option<i64>,
    pub last_update: Option<i64>,
    pub label: Option<String>,
    pub location: Option<String>,
    pub user_comment: Option<String>,
    pub heat: Option<f64>,
    pub spam_confidence: Option<i64>,
    pub calls: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentInfo {
    pub rating: Option<String>,
    pub comment: Option<String>,
    pub created: Option<i64>,
    pub up: Option<i64>,
    pub down: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingEntry {
    pub key: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub phone_id: Option<String>,
    pub info: Option<PhoneInfo>,
    pub top_rating: Option<String>,
    pub ratings: Option<Vec<RatingEntry>>,
    pub comments: Option<Vec<CommentInfo>>,
    pub searches: Option<Vec<i64>>,
    pub ai_summary: Option<String>,
    pub related_numbers: Option<Vec<String>>,
    pub prev: Option<String>,
    pub next: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonalizedNumber {
    pub phone: String,
    pub label: Option<String>,
    pub comment: Option<String>,
    pub rating: Option<String>,
    pub created: Option<i64>,
    pub wildcard: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberList {
    pub numbers: Vec<PersonalizedNumber>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistNumber {
    pub phone: String,
    pub rating: Option<String>,
    pub votes: Option<i64>,
    pub last_activity: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlocklistResponse {
    pub numbers: Vec<BlocklistNumber>,
    pub version: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RangeMatch {
    pub prefix: String,
    pub votes: Option<i64>,
    pub cnt: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrefixCheckResult {
    pub numbers: Vec<PhoneInfo>,
    pub range10: Option<Vec<RangeMatch>>,
    pub range100: Option<Vec<RangeMatch>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RateRequest {
    pub phone: String,
    pub rating: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dial_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunitySpamEntry {
    pub phone: String,
    pub val1: String,
    pub val2: String,
    pub last_activity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityStatus {
    pub active_blocklist_count: Option<usize>,
    pub recent_searches: Vec<CommunitySpamEntry>,
    pub recent_reports: Vec<CommunitySpamEntry>,
    pub newest_blocklist: Vec<CommunitySpamEntry>,
    pub top_spammers: Vec<CommunitySpamEntry>,
    pub most_searched: Vec<CommunitySpamEntry>,
}


