use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use crate::model::domain::MappingRow;
use super::styles;

/// Write the Data Traceability tab (データ来源追跡)
pub fn write_data_trace(
    sheet: &mut Worksheet,
    mappings: &[MappingRow],
) -> Result<()> {
    sheet.set_name("DataTrace")?;

    let header = styles::header_format();
    let cell = styles::cell_format();

    // Column widths
    let widths = [8.0, 35.0, 15.0, 15.0, 15.0, 20.0, 12.0, 25.0];
    for (i, w) in widths.iter().enumerate() {
        sheet.set_column_width(i as u16, *w)?;
    }

    // Headers
    let headers = [
        "Domain", "DEFINITION", "SDTM Variable", "Raw File",
        "Raw Field", "EDC Label", "Operation", "Parameter",
    ];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    let mut row = 1u32;

    for m in mappings {
        sheet.write_string_with_format(row, 0, &m.domain, &cell)?;
        sheet.write_string_with_format(row, 1, &m.definition, &cell)?;
        sheet.write_string_with_format(row, 2, &m.variable, &cell)?;
        sheet.write_string_with_format(row, 3, &m.filename, &cell)?;
        sheet.write_string_with_format(row, 4, &m.fieldname, &cell)?;
        sheet.write_string_with_format(row, 5, &m.label, &cell)?;
        sheet.write_string_with_format(row, 6, &m.oper_type, &cell)?;
        sheet.write_string_with_format(row, 7, &m.parameter, &cell)?;
        row += 1;
    }

    Ok(())
}
