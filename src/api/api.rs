use once_cell::sync::Lazy;
use reqwest::blocking;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReverseIpError {
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),
    #[error("API limit exceeded")]
    ApiLimitExceeded,
    #[error("No domains found")]
    NoDomainsFound,
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    ip_api_url: String,
    #[allow(unused)]
    viewdns_api_key: String,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            ip_api_url: "https://ip-api.com/json/".to_string(),
            viewdns_api_key: "".to_string(),
        }
    }
}

impl ApiConfig {
    pub fn ip_api_lookup(&self, ip: &str) -> Result<Vec<String>, ReverseIpError> {
        let url = format!("{}{}", self.ip_api_url, ip);
        let response = blocking::get(&url)?.text()?;

        if response.contains("API count exceeded") {
            return Err(ReverseIpError::ApiLimitExceeded);
        }

        let domains: Vec<String> = response
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.trim().to_string())
            .collect();

        if domains.is_empty() {
            Err(ReverseIpError::NoDomainsFound)
        } else {
            Ok(domains)
        }
    }
}

pub static API_CONFIG: Lazy<ApiConfig> = Lazy::new(|| {
    ApiConfig::default()
});
