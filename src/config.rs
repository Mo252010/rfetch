use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub general: General,
    #[serde(default)]
    pub theme: Theme,
    #[serde(default = "default_modules")]
    pub modules: Vec<Module>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General::default(),
            theme: Theme::default(),
            modules: default_modules(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct General {
    pub refresh_ms: u64,
    pub border: bool,
    pub rounded: bool,
    pub show_logo: bool,
    pub title: String,
}

impl Default for General {
    fn default() -> Self {
        Self {
            refresh_ms: 1000,
            border: true,
            rounded: true,
            show_logo: true,
            title: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub accent: String,
    pub label: String,
    pub value: String,
    pub title: String,
    pub border: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            accent: "cyan".into(),
            label: "green".into(),
            value: "white".into(),
            title: "cyan".into(),
            border: "darkgray".into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleType {
    Break,
    Header,
    Cpu,
    Memory,
    Swap,
    Gpu,
    Storage,
    Uptime,
    Kernel,
    Os,
    Host,
    Load,
}

impl ModuleType {
    pub const ALL: [ModuleType; 12] = [
        ModuleType::Header,
        ModuleType::Cpu,
        ModuleType::Memory,
        ModuleType::Swap,
        ModuleType::Gpu,
        ModuleType::Storage,
        ModuleType::Uptime,
        ModuleType::Kernel,
        ModuleType::Os,
        ModuleType::Host,
        ModuleType::Load,
        ModuleType::Break,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleType::Break => "break",
            ModuleType::Header => "header",
            ModuleType::Cpu => "cpu",
            ModuleType::Memory => "memory",
            ModuleType::Swap => "swap",
            ModuleType::Gpu => "gpu",
            ModuleType::Storage => "storage",
            ModuleType::Uptime => "uptime",
            ModuleType::Kernel => "kernel",
            ModuleType::Os => "os",
            ModuleType::Host => "host",
            ModuleType::Load => "load",
        }
    }

}

impl fmt::Display for ModuleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    #[serde(rename = "type")]
    pub kind: ModuleType,
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default = "default_bar_width")]
    pub bar_width: u16,
    #[serde(default)]
    pub mounts: Vec<String>,
}

impl Module {
    pub fn new(kind: ModuleType) -> Self {
        Self {
            kind,
            enabled: true,
            label: None,
            bar_width: default_bar_width(),
            mounts: Vec::new(),
        }
    }

    pub fn label_or(&self, fallback: &str) -> String {
        self.label.clone().unwrap_or_else(|| fallback.to_string())
    }
}

fn default_true() -> bool {
    true
}

fn default_bar_width() -> u16 {
    28
}

fn default_modules() -> Vec<Module> {
    vec![
        Module::new(ModuleType::Header),
        Module::new(ModuleType::Cpu),
        Module::new(ModuleType::Memory),
        Module::new(ModuleType::Gpu),
        Module::new(ModuleType::Storage),
        Module::new(ModuleType::Uptime),
    ]
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("rfetch")
        .join("config.toml")
}

pub fn load() -> Result<Config> {
    let path = config_path();
    if !path.exists() {
        let cfg = Config::default();
        save(&cfg)?;
        return Ok(cfg);
    }
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    let cfg: Config = toml::from_str(&text)
        .with_context(|| format!("parsing {}", path.display()))?;
    Ok(cfg)
}

pub fn save(cfg: &Config) -> Result<()> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let text = toml::to_string_pretty(cfg)?;
    std::fs::write(&path, text).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}
