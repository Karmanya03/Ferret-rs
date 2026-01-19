use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Configuration for Ferret
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FerretConfig {
    /// Custom file type mappings
    #[serde(default)]
    pub file_types: HashMap<String, Vec<String>>,

    /// Custom organization rules
    #[serde(default)]
    pub organization: OrganizationConfig,

    /// Performance settings
    #[serde(default)]
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrganizationConfig {
    /// Custom category definitions
    #[serde(default)]
    pub categories: HashMap<String, Vec<String>>,

    /// Default organization method
    #[serde(default = "default_org_method")]
    pub default_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Number of threads for parallel operations
    #[serde(default = "default_threads")]
    pub threads: usize,

    /// Maximum file size to hash (in MB) for duplicate detection
    #[serde(default = "default_max_hash_size")]
    pub max_hash_size_mb: u64,
}

fn default_org_method() -> String {
    "type".to_string()
}

fn default_threads() -> usize {
    num_cpus::get()
}

fn default_max_hash_size() -> u64 {
    1024 // 1GB
}

impl Default for FerretConfig {
    fn default() -> Self {
        Self {
            file_types: default_file_types(),
            organization: OrganizationConfig::default(),
            performance: PerformanceConfig::default(),
        }
    }
}

impl Default for OrganizationConfig {
    fn default() -> Self {
        Self {
            categories: default_file_types(),
            default_method: default_org_method(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            threads: default_threads(),
            max_hash_size_mb: default_max_hash_size(),
        }
    }
}

impl FerretConfig {
    /// Load config from file, or create default if not exists
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents = fs::read_to_string(&config_path)
                .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

            let config: FerretConfig =
                toml::from_str(&contents).with_context(|| "Failed to parse config file")?;

            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save config to file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        let contents =
            toml::to_string_pretty(self).with_context(|| "Failed to serialize config")?;

        fs::write(&config_path, contents)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }

    /// Get the config file path
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = if cfg!(windows) {
            dirs::config_dir()
                .context("Could not find config directory")?
                .join("ferret")
        } else {
            dirs::home_dir()
                .context("Could not find home directory")?
                .join(".config")
                .join("ferret")
        };

        Ok(config_dir.join("config.toml"))
    }

    /// Initialize config file with defaults
    pub fn init() -> Result<PathBuf> {
        let config = Self::default();
        config.save()?;
        Self::config_path()
    }

    /// Get file category based on extension
    pub fn get_category(&self, extension: &str) -> Option<String> {
        let ext_lower = extension.to_lowercase();

        for (category, extensions) in &self.file_types {
            if extensions.iter().any(|e| e.to_lowercase() == ext_lower) {
                return Some(category.clone());
            }
        }

        None
    }
}

/// Default file type mappings
fn default_file_types() -> HashMap<String, Vec<String>> {
    let mut types = HashMap::new();

    types.insert(
        "documents".to_string(),
        vec![
            "pdf", "doc", "docx", "txt", "md", "rtf", "odt", "tex", "epub",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );

    types.insert(
        "images".to_string(),
        vec![
            "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tiff", "raw",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );

    types.insert(
        "videos".to_string(),
        vec![
            "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );

    types.insert(
        "audio".to_string(),
        vec![
            "mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "opus", "ape",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );

    types.insert(
        "archives".to_string(),
        vec!["zip", "tar", "gz", "7z", "rar", "bz2", "xz", "tgz", "iso"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    types.insert(
        "code".to_string(),
        vec![
            "rs", "py", "js", "ts", "c", "cpp", "h", "hpp", "go", "java", "rb", "php", "swift",
            "kt",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );

    types.insert(
        "web".to_string(),
        vec![
            "html", "css", "scss", "sass", "less", "json", "xml", "yaml", "yml",
        ]
        .into_iter()
        .map(String::from)
        .collect(),
    );

    types.insert(
        "spreadsheets".to_string(),
        vec!["xls", "xlsx", "csv", "ods"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    types.insert(
        "presentations".to_string(),
        vec!["ppt", "pptx", "odp", "key"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    types.insert(
        "executables".to_string(),
        vec!["exe", "msi", "app", "deb", "rpm", "apk", "dmg"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    types.insert(
        "databases".to_string(),
        vec!["db", "sqlite", "sqlite3", "sql", "mdb"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    types.insert(
        "fonts".to_string(),
        vec!["ttf", "otf", "woff", "woff2", "eot"]
            .into_iter()
            .map(String::from)
            .collect(),
    );

    types
}

// Add num_cpus as a dependency helper
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}
