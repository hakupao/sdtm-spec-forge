use std::collections::HashMap;
use std::path::Path;
use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};
use crate::model::domain::MappingRow;

/// Read Mapping sheet from OperationConf Excel
pub fn read_mappings(path: &Path) -> Result<Vec<MappingRow>> {
    let mut workbook: Xlsx<_> = open_workbook(path)
        .with_context(|| format!("Cannot open mapping file: {}", path.display()))?;

    let range = workbook
        .worksheet_range("Mapping")
        .with_context(|| "Sheet 'Mapping' not found")?;

    let mut mappings = Vec::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        }

        let get = |idx: usize| -> String {
            row.get(idx)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default()
        };

        let domain = get(1);
        if domain.is_empty() {
            continue;
        }

        mappings.push(MappingRow {
            definition: get(0),
            domain,
            variable: get(2),
            terminology: get(3),
            filename: get(5),
            fieldname: get(6),
            label: get(7),
            oper_type: get(8),
            parameter: get(9),
            notes: get(10),
        });
    }

    Ok(mappings)
}

/// Read DomainsSetting sheet to get sort keys
pub fn read_domain_settings(path: &Path) -> Result<HashMap<String, String>> {
    let mut workbook: Xlsx<_> = open_workbook(path)
        .with_context(|| format!("Cannot open mapping file: {}", path.display()))?;

    let range = workbook
        .worksheet_range("DomainsSetting")
        .with_context(|| "Sheet 'DomainsSetting' not found")?;

    let mut settings = HashMap::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        }

        let get = |idx: usize| -> String {
            row.get(idx)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default()
        };

        let domain = get(0);
        let sort_keys = get(2);
        if !domain.is_empty() {
            settings.insert(domain, sort_keys);
        }
    }

    Ok(settings)
}
