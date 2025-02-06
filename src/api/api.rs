use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking;
use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReverseipError {
    #[error("HTTP request failed: {0}")]
    HttpRequestFailed(#[from] reqwest::Error),

    #[error("JSON parsing failed: {0}")]
    JsonParsingFailed(#[from] serde_json::Error),

    #[allow(unused)]
    #[error("API limit exceeded")]
    ApiLimitExceeded,

    #[error("No company names found")]
    NoCompanyNamesFound,
}

#[derive(Debug, Clone)]
pub struct ApiConfig {
    rdap_api_url: String,
}

impl Default for ApiConfig {
    fn default() -> Self {Self {rdap_api_url: "https://rdap.arin.net/registry/ip/".to_string(),}}
}

impl ApiConfig {
    pub fn ip_api_lookup(&self, ip: &str) -> Result<Vec<String>, ReverseipError> {
        let url = format!("{}{}", self.rdap_api_url, ip);
        let response = blocking::get(&url)?.text()?;
        let company_names = extract_company_names(&response)?;
        Ok(company_names)
    }
}

fn extract_company_names(json_str: &str) -> Result<Vec<String>, ReverseipError> {
    let parsed: Value = serde_json::from_str(json_str).map_err(ReverseipError::JsonParsingFailed)?;
    let mut companies = Vec::new();
    let company_regex = Regex::new(r"\b([A-Z][a-z]+(?:\s+[A-Z][a-z]+)*)\b").unwrap();

    if let Some(nome) = parsed.get("nome").and_then(Value::as_str) {
        for cap in company_regex.captures_iter(nome) {companies.push(cap[1].to_string());}
    }

    if let Some(entidades) = parsed.get("entidades").and_then(Value::as_array) {
        for entidade in entidades {
            if let Some(vcard_array) = entidade.get("vcardArray").and_then(Value::as_array) {
                if let Some(vcard_items) = vcard_array.get(1).and_then(Value::as_array) {
                    for item in vcard_items {
                        if let Some(valor) = item.get(3).and_then(Value::as_str) {
                            for cap in company_regex.captures_iter(valor) {
                                companies.push(cap[1].to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    if let Some(observacoes) = parsed.get("observações").and_then(Value::as_array) {
        for obs in observacoes {
            if let Some(descs) = obs.get("descrição").and_then(Value::as_array) {
                for desc in descs {
                    if let Some(desc_str) = desc.as_str() {
                        for cap in company_regex.captures_iter(desc_str) {
                            companies.push(cap[1].to_string());
                        }
                    }
                }
            }
        }
    }

    companies.sort();
    companies.dedup();

    if companies.is_empty() {
        Err(ReverseipError::NoCompanyNamesFound)
    } else {
        Ok(companies)
    }
}

pub static API_CONFIG: Lazy<ApiConfig> = Lazy::new(|| ApiConfig::default());
