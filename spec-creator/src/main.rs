use std::path::Path;
use clap::Parser;
use anyhow::{Context, Result};
use serde::Deserialize;
use sdtm_spec_creator::SpecConfig;

#[derive(Parser, Debug)]
#[command(name = "sdtm-spec-creator")]
#[command(about = "SDTM Dataset Specification Excel Generator")]
struct Cli {
    /// Path to config TOML file (if provided, other args are optional)
    #[arg(long, short = 'c')]
    config: Option<String>,

    /// Path to SDTM CSV data directory
    #[arg(long)]
    data_dir: Option<String>,

    /// Path to SDTMIG v3.4 Excel file
    #[arg(long)]
    sdtmig: Option<String>,

    /// Path to SDTM Terminology Excel file
    #[arg(long)]
    terminology: Option<String>,

    /// Path to OperationConf.xlsx mapping file
    #[arg(long)]
    mapping: Option<String>,

    /// Output directory
    #[arg(long)]
    output: Option<String>,

    /// Study ID
    #[arg(long)]
    study_id: Option<String>,
}

#[derive(Deserialize)]
struct TomlConfig {
    study_id: String,
    data_dir: String,
    sdtmig: String,
    terminology: String,
    mapping: String,
    #[serde(default = "default_output")]
    output: String,
}

fn default_output() -> String {
    "./output".to_string()
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = if let Some(config_path) = &cli.config {
        // Load from TOML, then CLI args override
        let toml_str = std::fs::read_to_string(config_path)
            .with_context(|| format!("設定ファイルの読取に失敗: {}", config_path))?;

        // Resolve paths relative to config file location
        let config_dir = Path::new(config_path)
            .parent()
            .unwrap_or(Path::new("."));

        let toml: TomlConfig = toml::from_str(&toml_str)
            .with_context(|| format!("設定ファイルの解析に失敗: {}", config_path))?;

        let resolve = |p: &str| -> String {
            let path = Path::new(p);
            if path.is_absolute() {
                p.to_string()
            } else {
                config_dir.join(p).to_string_lossy().to_string()
            }
        };

        SpecConfig {
            study_id:         cli.study_id.unwrap_or(toml.study_id),
            data_dir:         cli.data_dir.unwrap_or_else(|| resolve(&toml.data_dir)),
            sdtmig_path:      cli.sdtmig.unwrap_or_else(|| resolve(&toml.sdtmig)),
            terminology_path: cli.terminology.unwrap_or_else(|| resolve(&toml.terminology)),
            mapping_path:     cli.mapping.unwrap_or_else(|| resolve(&toml.mapping)),
            output_dir:       cli.output.unwrap_or_else(|| resolve(&toml.output)),
        }
    } else {
        // Pure CLI mode: all args required
        SpecConfig {
            data_dir:         cli.data_dir.context("--data-dir is required")?,
            sdtmig_path:      cli.sdtmig.context("--sdtmig is required")?,
            terminology_path: cli.terminology.context("--terminology is required")?,
            mapping_path:     cli.mapping.context("--mapping is required")?,
            output_dir:       cli.output.unwrap_or_else(|| "./output".to_string()),
            study_id:         cli.study_id.context("--study-id is required")?,
        }
    };

    sdtm_spec_creator::run(&config)
}
