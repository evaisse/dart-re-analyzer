use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::error::{AnalyzerError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzerConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    
    #[serde(default)]
    pub style_rules: RuleSetConfig,
    
    #[serde(default)]
    pub runtime_rules: RuleSetConfig,
    
    #[serde(default = "default_max_line_length")]
    pub max_line_length: usize,
    
    #[serde(default = "default_parallel")]
    pub parallel: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSetConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    
    #[serde(default)]
    pub disabled_rules: Vec<String>,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            exclude_patterns: vec![
                ".dart_tool/**".to_string(),
                "build/**".to_string(),
                ".pub/**".to_string(),
                "packages/**".to_string(),
            ],
            style_rules: RuleSetConfig::default(),
            runtime_rules: RuleSetConfig::default(),
            max_line_length: 120,
            parallel: true,
        }
    }
}

impl Default for RuleSetConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            disabled_rules: Vec::new(),
        }
    }
}

fn default_enabled() -> bool {
    true
}

fn default_max_line_length() -> usize {
    120
}

fn default_parallel() -> bool {
    true
}

impl AnalyzerConfig {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: AnalyzerConfig = serde_json::from_str(&content)
            .map_err(|e| AnalyzerError::Config(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }
    
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| AnalyzerError::Config(format!("Failed to serialize config: {}", e)))?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    pub fn is_rule_enabled(&self, rule_name: &str, is_style: bool) -> bool {
        let rule_set = if is_style {
            &self.style_rules
        } else {
            &self.runtime_rules
        };
        
        rule_set.enabled && !rule_set.disabled_rules.contains(&rule_name.to_string())
    }
}
