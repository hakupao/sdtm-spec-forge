use std::collections::HashMap;
use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use crate::model::domain::CodelistMatch;
use super::styles;

/// Write the Codelist tab (コードリスト一覧)
/// Group by (domain, variable) and combine codelist codes into "C66727; C114118" format
pub fn write_codelist(
    sheet: &mut Worksheet,
    matches: &[CodelistMatch],
) -> Result<()> {
    sheet.set_name("Codelist")?;

    let header = styles::header_format();
    let cell = styles::cell_format();

    // Column widths: Domain, Variable Name, Codelist, Value, CDISC Definition
    let widths = [8.0, 15.0, 30.0, 25.0, 50.0];
    for (i, w) in widths.iter().enumerate() {
        sheet.set_column_width(i as u16, *w)?;
    }

    // Headers
    let headers = [
        "Domain", "Variable Name", "Codelist", "Value", "CDISC Definition",
    ];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    // Build codelist code lookup: (domain, variable) -> combined C-codes
    let mut var_codes: HashMap<(String, String), Vec<String>> = HashMap::new();
    for m in matches {
        let key = (m.domain.clone(), m.variable.clone());
        let codes = var_codes.entry(key).or_default();
        if !m.codelist_code.is_empty() && !codes.contains(&m.codelist_code) {
            codes.push(m.codelist_code.clone());
        }
    }

    // Deduplicate matches by (domain, variable, actual_value)
    let mut seen: HashMap<(String, String, String), bool> = HashMap::new();
    let mut row = 1u32;

    for m in matches {
        let dedup_key = (m.domain.clone(), m.variable.clone(), m.actual_value.clone());
        if seen.contains_key(&dedup_key) {
            continue;
        }
        seen.insert(dedup_key, true);

        let var_key = (m.domain.clone(), m.variable.clone());
        let codelist_str = var_codes
            .get(&var_key)
            .map(|codes| codes.join("; "))
            .unwrap_or_default();

        sheet.write_string_with_format(row, 0, &m.domain, &cell)?;
        sheet.write_string_with_format(row, 1, &m.variable, &cell)?;
        sheet.write_string_with_format(row, 2, &codelist_str, &cell)?;
        sheet.write_string_with_format(row, 3, &m.actual_value, &cell)?;
        sheet.write_string_with_format(row, 4, &m.cdisc_definition, &cell)?;
        row += 1;
    }

    Ok(())
}
