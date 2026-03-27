use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use crate::model::domain::MappingRow;
use super::styles;

/// Build a human-readable "EDC Source" string: Label (Filename.csv→Fieldname)
/// Returns empty string for DEF (Assigned value) rows.
fn format_edc_source(m: &MappingRow) -> String {
    // Leave blank when derivation is Assigned value
    if m.oper_type == "DEF" {
        return String::new();
    }

    let has_label = !m.label.is_empty();
    let has_field = !m.fieldname.is_empty();
    let has_file = !m.filename.is_empty();

    // Build the parenthetical: (Filename.csv→Fieldname)
    let path = match (has_file, has_field) {
        (true, true) => format!("{}.csv\u{2192}{}", m.filename, m.fieldname),
        (true, false) => format!("{}.csv", m.filename),
        (false, true) => m.fieldname.clone(),
        (false, false) => String::new(),
    };

    match (has_label, !path.is_empty()) {
        (true, true) => format!("{} ({})", m.label, path),
        (true, false) => m.label.clone(),
        (false, true) => path,
        (false, false) => String::new(),
    }
}

/// Build a human-readable derivation rule from oper_type + parameter.
fn format_derivation_rule(m: &MappingRow) -> String {
    let param = &m.parameter;
    match m.oper_type.as_str() {
        "DEF" => {
            if param.is_empty() {
                "Assigned value".to_string()
            } else {
                format!("Assigned value: {}", param)
            }
        }
        "FIX" => "Direct mapping from EDC".to_string(),
        "CDL" => {
            if param.is_empty() {
                "Codelist mapping".to_string()
            } else {
                format!("Codelist mapping ({})", param)
            }
        }
        "FLG" => {
            if param.is_empty() {
                "Conditional derivation".to_string()
            } else {
                // param format: "VALUE:RESULT" or multi-line
                let readable: Vec<&str> = param.split('\n').collect();
                if readable.len() > 1 {
                    let conditions: Vec<String> = readable
                        .iter()
                        .filter(|s| !s.is_empty())
                        .map(|s| {
                            if let Some((k, v)) = s.split_once(':') {
                                format!("{} → {}", k.trim(), v.trim())
                            } else {
                                s.trim().to_string()
                            }
                        })
                        .collect();
                    format!("Conditional: {}", conditions.join("; "))
                } else if let Some((k, v)) = param.split_once(':') {
                    format!("Conditional: {} → {}", k.trim(), v.trim())
                } else {
                    format!("Conditional: {}", param)
                }
            }
        }
        "SEL" => {
            if param.is_empty() {
                "Row selection filter".to_string()
            } else if let Some((k, v)) = param.split_once(':') {
                format!("Filter: {} = {}", k.trim(), v.trim())
            } else {
                format!("Filter: {}", param)
            }
        }
        "" => String::new(),
        other => {
            if param.is_empty() {
                other.to_string()
            } else {
                format!("{}: {}", other, param)
            }
        }
    }
}

/// Write the Data Traceability tab (データ来源追跡)
pub fn write_data_trace(
    sheet: &mut Worksheet,
    mappings: &[MappingRow],
) -> Result<()> {
    sheet.set_name("DataTrace")?;

    let header = styles::header_format();
    let cell = styles::cell_format();

    // Column widths
    let widths = [8.0, 35.0, 15.0, 30.0, 35.0];
    for (i, w) in widths.iter().enumerate() {
        sheet.set_column_width(i as u16, *w)?;
    }

    // Headers
    let headers = [
        "Domain", "DEFINITION", "SDTM Variable",
        "EDC Source", "Derivation Rule",
    ];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    let mut row = 1u32;

    for m in mappings {
        let edc_source = format_edc_source(m);
        let derivation = format_derivation_rule(m);

        sheet.write_string_with_format(row, 0, &m.domain, &cell)?;
        sheet.write_string_with_format(row, 1, &m.definition, &cell)?;
        sheet.write_string_with_format(row, 2, &m.variable, &cell)?;
        sheet.write_string_with_format(row, 3, &edc_source, &cell)?;
        sheet.write_string_with_format(row, 4, &derivation, &cell)?;
        row += 1;
    }

    Ok(())
}
