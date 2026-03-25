use std::path::Path;
use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};
use crate::model::domain::TerminologyEntry;

/// Read SDTM Terminology Excel
pub fn read_terminology(path: &Path) -> Result<Vec<TerminologyEntry>> {
    let mut workbook: Xlsx<_> = open_workbook(path)
        .with_context(|| format!("Cannot open Terminology: {}", path.display()))?;

    // Find the sheet (name contains "SDTM Terminology")
    let sheet_names = workbook.sheet_names().to_vec();
    let sheet_name = sheet_names
        .iter()
        .find(|n| n.contains("SDTM Terminology"))
        .cloned()
        .unwrap_or_else(|| sheet_names[0].clone());

    let range = workbook
        .worksheet_range(&sheet_name)
        .with_context(|| format!("Cannot read sheet: {}", sheet_name))?;

    let mut entries = Vec::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue; // skip header
        }

        let get = |idx: usize| -> String {
            row.get(idx)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default()
        };

        let code = get(0);
        if code.is_empty() {
            continue;
        }

        entries.push(TerminologyEntry {
            code,
            codelist_code: get(1),
            codelist_name: get(3),
            submission_value: get(4),
            cdisc_definition: get(6),
        });
    }

    Ok(entries)
}
