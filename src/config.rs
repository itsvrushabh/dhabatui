use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DhabaProfile {
    #[serde(default = "default_name")]
    pub name: String,
    #[serde(default = "default_tagline")]
    pub tagline: String,
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_phone")]
    pub phone: String,
    #[serde(default = "default_gstin")]
    pub gstin: String,
}

fn default_name() -> String {
    "DHABA TUI".to_string()
}
fn default_tagline() -> String {
    "Thank You! Visit Again!".to_string()
}
fn default_address() -> String {
    "National Highway Eatery".to_string()
}
fn default_phone() -> String {
    "+91 98765 43210".to_string()
}
fn default_gstin() -> String {
    "27AAAAA0000A1Z5".to_string()
}

impl Default for DhabaProfile {
    fn default() -> Self {
        Self {
            name: default_name(),
            tagline: default_tagline(),
            address: default_address(),
            phone: default_phone(),
            gstin: default_gstin(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaxSettings {
    #[serde(default = "default_gst_enabled")]
    pub enable_gst: bool,
    /// CGST rate in tenths of a percent (e.g. 25 = 2.5%)
    #[serde(default = "default_tax_rate")]
    pub cgst_rate_permille: u32,
    /// SGST rate in tenths of a percent (e.g. 25 = 2.5%)
    #[serde(default = "default_tax_rate")]
    pub sgst_rate_permille: u32,
}

fn default_gst_enabled() -> bool {
    true
}
fn default_tax_rate() -> u32 {
    25
}

impl Default for TaxSettings {
    fn default() -> Self {
        Self {
            enable_gst: default_gst_enabled(),
            cgst_rate_permille: default_tax_rate(),
            sgst_rate_permille: default_tax_rate(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TableConfig {
    #[serde(default = "default_table_count")]
    pub count: usize,
    #[serde(default)]
    pub custom_labels: Vec<String>,
}

fn default_table_count() -> usize {
    8
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            count: default_table_count(),
            custom_labels: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WebConfig {
    #[serde(default = "default_web_enabled")]
    pub enabled: bool,
    #[serde(default = "default_web_port")]
    pub port: u16,
}

fn default_web_enabled() -> bool {
    true
}
fn default_web_port() -> u16 {
    8080
}

impl Default for WebConfig {
    fn default() -> Self {
        Self {
            enabled: default_web_enabled(),
            port: default_web_port(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default)]
    pub dhaba: DhabaProfile,
    #[serde(default)]
    pub tax: TaxSettings,
    #[serde(default)]
    pub tables: TableConfig,
    #[serde(default)]
    pub web: WebConfig,
}

fn dirs_home_config() -> Option<PathBuf> {
    if let Ok(config_home) = std::env::var("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(config_home));
    }
    if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
        return Some(PathBuf::from(home).join(".config"));
    }
    None
}

impl AppConfig {
    pub fn load() -> Self {
        // 1. Check local directory ./dhabatui.toml
        if let Ok(content) = fs::read_to_string("dhabatui.toml") {
            if let Ok(cfg) = toml::from_str::<AppConfig>(&content) {
                tracing::info!("Loaded configuration from local ./dhabatui.toml");
                return cfg;
            }
        }

        // 2. Check ~/.config/dhabatui/config.toml
        if let Some(mut config_path) = dirs_home_config() {
            config_path.push("dhabatui");
            config_path.push("config.toml");
            if let Ok(content) = fs::read_to_string(&config_path) {
                if let Ok(cfg) = toml::from_str::<AppConfig>(&content) {
                    tracing::info!(path = %config_path.display(), "Loaded configuration from user config");
                    return cfg;
                }
            }
        }

        // 3. Fallback to sensible defaults
        Self::default()
    }

    pub fn generate_sample_toml() -> &'static str {
        r#"# DhabaTUI Configuration File (dhabatui.toml)

[dhaba]
name = "SHER-E-PUNJAB DHABA"
tagline = "Taste of Highway • Desi Ghee Swad!"
address = "GT Road, Near Mile 142, Ambala"
phone = "+91 98765 43210"
gstin = "06AAAAA0000A1Z5"

[tax]
enable_gst = true
# Tax rate in permille (25 = 2.5% CGST and 2.5% SGST)
cgst_rate_permille = 25
sgst_rate_permille = 25

[tables]
count = 8
# Optional custom table labels (e.g. Khat 1, Garden Table, AC Hall)
# custom_labels = ["Khat 1", "Khat 2", "Family 1", "Family 2", "Garden A", "Garden B"]

[web]
enabled = true
port = 8080
"#
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = AppConfig::default();
        assert_eq!(cfg.dhaba.name, "DHABA TUI");
        assert!(cfg.tax.enable_gst);
        assert_eq!(cfg.tables.count, 8);
        assert_eq!(cfg.web.port, 8080);
    }

    #[test]
    fn test_parse_sample_toml() {
        let sample = AppConfig::generate_sample_toml();
        let cfg: AppConfig = toml::from_str(sample).unwrap();
        assert_eq!(cfg.dhaba.name, "SHER-E-PUNJAB DHABA");
        assert_eq!(cfg.dhaba.phone, "+91 98765 43210");
        assert_eq!(cfg.tax.cgst_rate_permille, 25);
    }
}
