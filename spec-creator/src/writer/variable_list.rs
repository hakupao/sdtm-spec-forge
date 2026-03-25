use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use crate::model::domain::DomainData;
use super::styles;

/// Write a Variable List tab for one domain
pub fn write_variable_list(
    sheet: &mut Worksheet,
    domain: &DomainData,
) -> Result<()> {
    sheet.set_name(&domain.name)?;

    let header = styles::header_format();
    let cell = styles::cell_format();
    let custom = styles::custom_var_format();

    // Column widths (removed Origin, removed サンプル値)
    let widths = [5.0, 18.0, 35.0, 8.0, 8.0, 10.0, 15.0, 20.0, 50.0, 35.0, 25.0];
    for (i, w) in widths.iter().enumerate() {
        sheet.set_column_width(i as u16, *w)?;
    }

    // Headers (removed Origin)
    let headers = [
        "No.", "Variable Name", "Variable Label", "Type", "Length",
        "Core", "Role", "Codelist",
        "CDISC Notes", "データ説明", "備考",
    ];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    // Data rows
    for (i, var) in domain.variables.iter().enumerate() {
        let row = (i + 1) as u32;
        let fmt = if var.is_custom { &custom } else { &cell };

        sheet.write_number_with_format(row, 0, (i + 1) as f64, fmt)?;
        sheet.write_string_with_format(row, 1, &var.name, fmt)?;
        sheet.write_string_with_format(row, 2, &var.label, fmt)?;
        sheet.write_string_with_format(row, 3, &var.var_type, fmt)?;
        sheet.write_number_with_format(row, 4, var.length as f64, fmt)?;
        sheet.write_string_with_format(row, 5, &var.core, fmt)?;
        sheet.write_string_with_format(row, 6, &var.role, fmt)?;
        sheet.write_string_with_format(row, 7, &var.codelist, fmt)?;
        sheet.write_string_with_format(row, 8, &var.cdisc_notes, fmt)?;
        sheet.write_string_with_format(row, 9, &var.definition, fmt)?;
        sheet.write_string_with_format(row, 10, "", fmt)?; // 備考 (manual)
    }

    Ok(())
}
