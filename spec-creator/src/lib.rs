pub mod model;
pub mod reader;
pub mod analyzer;
pub mod writer;

use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};

use model::domain::DomainData;
use reader::{csv_reader, sdtmig_reader, terminology_reader, mapping_reader};
use analyzer::{tree_builder, stats, codelist_matcher};

/// Configuration for the spec generator
pub struct SpecConfig {
    pub data_dir: String,
    pub sdtmig_path: String,
    pub terminology_path: String,
    pub mapping_path: String,
    pub output_dir: String,
    pub study_id: String,
}

/// Main entry point: run the full spec generation pipeline
pub fn run(config: &SpecConfig) -> Result<()> {
    println!("=== SDTM Dataset Specification Creator ===");
    println!();

    // 1. Read all input files
    println!("[1/5] 入力ファイル読取中...");

    let csv_datasets = csv_reader::read_all_csvs(Path::new(&config.data_dir))
        .context("CSVデータセットの読取に失敗")?;
    println!("  CSV datasets: {} domains loaded", csv_datasets.len());

    let sdtmig_vars = sdtmig_reader::read_sdtmig_variables(Path::new(&config.sdtmig_path))
        .context("SDTMIG変数メタデータの読取に失敗")?;
    println!("  SDTMIG variables: {} entries", sdtmig_vars.len());

    let sdtmig_datasets = sdtmig_reader::read_sdtmig_datasets(Path::new(&config.sdtmig_path))
        .context("SDTMIGデータセットメタデータの読取に失敗")?;

    let terminology = terminology_reader::read_terminology(Path::new(&config.terminology_path))
        .context("CDISC Terminologyの読取に失敗")?;
    println!("  CDISC Terminology: {} entries", terminology.len());

    let mappings = mapping_reader::read_mappings(Path::new(&config.mapping_path))
        .context("マッピング設定の読取に失敗")?;
    println!("  Mapping rows: {}", mappings.len());

    let domain_settings = mapping_reader::read_domain_settings(Path::new(&config.mapping_path))
        .context("ドメイン設定の読取に失敗")?;

    // 2. Build dataset lookup
    let ds_lookup: HashMap<&str, &model::variable::SdtmigDataset> = sdtmig_datasets
        .iter()
        .map(|d| (d.domain.as_str(), d))
        .collect();

    // 3. Analyze each domain
    println!();
    println!("[2/5] データ分析中...");

    let mut analyzed_domains: Vec<DomainData> = Vec::new();

    for csv_data in &csv_datasets {
        let domain_name = &csv_data.domain;
        println!("  Analyzing {}...", domain_name);

        // Build tree
        let tree = tree_builder::build_domain_tree(csv_data, &mappings);

        if let Some(ref t) = tree {
            println!("    Tree: {} root nodes, hierarchy: {:?}",
                t.roots.len(), t.hierarchy_fields);
        }

        // Get sort keys
        let sort_keys = domain_settings
            .get(domain_name.as_str())
            .cloned()
            .unwrap_or_default();

        // Analyze
        let domain_data = stats::analyze_domain(
            csv_data,
            &sdtmig_vars,
            ds_lookup.get(domain_name.as_str()).copied(),
            &mappings,
            &sort_keys,
            tree,
        );

        println!("    Variables: {} (non-empty) / {} (total), Records: {}",
            domain_data.variables.len(),
            domain_data.all_variable_count,
            domain_data.record_count,
        );

        analyzed_domains.push(domain_data);
    }

    // 4. Match codelists
    println!();
    println!("[3/5] コードリストマッチング中...");
    let codelist_matches = codelist_matcher::match_codelists(&analyzed_domains, &terminology);
    println!("  Codelist matches: {}", codelist_matches.len());

    // 5. Generate Excel
    println!();
    println!("[4/5] Excel仕様書生成中...");

    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let date_compact = chrono::Local::now().format("%Y%m%d").to_string();
    let filename = format!("{}_SDTM_Dataset_Specification_{}.xlsx", config.study_id, date_compact);

    let output_dir = Path::new(&config.output_dir);
    std::fs::create_dir_all(output_dir).context("出力ディレクトリの作成に失敗")?;

    let output_path = output_dir.join(&filename);

    writer::excel_writer::generate_spec(
        &output_path,
        &config.study_id,
        &date,
        &analyzed_domains,
        &mappings,
        &codelist_matches,
    )?;

    println!();
    println!("[5/5] 完了！");
    println!("  出力ファイル: {}", output_path.display());

    Ok(())
}
