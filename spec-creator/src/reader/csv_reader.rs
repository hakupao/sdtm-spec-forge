use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

/// Raw CSV data for one domain
#[derive(Debug)]
pub struct CsvData {
    pub domain: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl CsvData {
    pub fn record_count(&self) -> usize {
        self.rows.len()
    }

    pub fn column_values(&self, col_name: &str) -> Vec<&str> {
        let idx = self.headers.iter().position(|h| h == col_name);
        match idx {
            Some(i) => self.rows.iter().map(|r| r[i].as_str()).collect(),
            None => Vec::new(),
        }
    }

    pub fn is_column_empty(&self, col_name: &str) -> bool {
        self.column_values(col_name)
            .iter()
            .all(|v| v.trim().is_empty())
    }

    pub fn unique_values(&self, col_name: &str) -> Vec<String> {
        let mut seen = indexmap::IndexSet::new();
        for v in self.column_values(col_name) {
            let trimmed = v.trim();
            if !trimmed.is_empty() {
                seen.insert(trimmed.to_string());
            }
        }
        seen.into_iter().collect()
    }

    pub fn max_length(&self, col_name: &str) -> usize {
        self.column_values(col_name)
            .iter()
            .map(|v| v.len())
            .max()
            .unwrap_or(0)
    }
}

/// Read all CSV files from a directory
pub fn read_all_csvs(dir: &Path) -> Result<Vec<CsvData>> {
    let mut datasets = Vec::new();

    let mut entries: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("Cannot read directory: {}", dir.display()))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("csv"))
        })
        .collect();

    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let path = entry.path();
        let domain = path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        let data = read_csv(&path, &domain)
            .with_context(|| format!("Failed to read CSV: {}", path.display()))?;
        datasets.push(data);
    }

    Ok(datasets)
}

fn read_csv(path: &Path, domain: &str) -> Result<CsvData> {
    // Read file content, strip UTF-8 BOM if present
    let content = fs::read(path)?;
    let text = if content.starts_with(&[0xEF, 0xBB, 0xBF]) {
        String::from_utf8_lossy(&content[3..]).to_string()
    } else {
        String::from_utf8_lossy(&content).to_string()
    };

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_reader(text.as_bytes());

    let headers: Vec<String> = rdr
        .headers()?
        .iter()
        .map(|h| h.to_string())
        .collect();

    let mut rows = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let row: Vec<String> = record.iter().map(|f| f.to_string()).collect();
        rows.push(row);
    }

    Ok(CsvData {
        domain: domain.to_string(),
        headers,
        rows,
    })
}
