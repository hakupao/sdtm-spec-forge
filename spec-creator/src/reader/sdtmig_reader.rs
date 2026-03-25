use std::path::Path;
use anyhow::{Context, Result};
use calamine::{open_workbook, Reader, Xlsx};
use crate::model::variable::{SdtmigDataset, SdtmigVariable};

/// Read SDTMIG Variables sheet
pub fn read_sdtmig_variables(path: &Path) -> Result<Vec<SdtmigVariable>> {
    let mut workbook: Xlsx<_> = open_workbook(path)
        .with_context(|| format!("Cannot open SDTMIG: {}", path.display()))?;

    let range = workbook
        .worksheet_range("Variables")
        .with_context(|| "Sheet 'Variables' not found")?;

    let mut variables = Vec::new();
    let mut order = 0;

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue; // skip header
        }

        let get = |idx: usize| -> String {
            row.get(idx)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default()
        };

        let domain = get(3);  // Dataset Name
        let name = get(4);    // Variable Name

        if domain.is_empty() || name.is_empty() {
            continue;
        }

        order += 1;

        variables.push(SdtmigVariable {
            domain,
            name,
            label: get(5),           // Variable Label
            var_type: get(8),        // Type
            core: String::new(),     // Will be derived from the data
            origin: String::new(),
            role: get(13),           // Role
            codelist_code: get(10),  // CDISC CT Codelist Code(s)
            cdisc_notes: get(14),    // CDISC Notes
            order,
        });
    }

    Ok(variables)
}

/// Read SDTMIG Datasets sheet
pub fn read_sdtmig_datasets(path: &Path) -> Result<Vec<SdtmigDataset>> {
    let mut workbook: Xlsx<_> = open_workbook(path)
        .with_context(|| format!("Cannot open SDTMIG: {}", path.display()))?;

    let range = workbook
        .worksheet_range("Datasets")
        .with_context(|| "Sheet 'Datasets' not found")?;

    let mut datasets = Vec::new();

    for (i, row) in range.rows().enumerate() {
        if i == 0 {
            continue;
        }

        let get = |idx: usize| -> String {
            row.get(idx)
                .map(|c| c.to_string().trim().to_string())
                .unwrap_or_default()
        };

        let domain = get(2);
        if domain.is_empty() {
            continue;
        }

        datasets.push(SdtmigDataset {
            domain,
            label: get(3),      // Dataset Label
            class: get(1),      // Class
            structure: get(6),  // Structure
        });
    }

    Ok(datasets)
}
