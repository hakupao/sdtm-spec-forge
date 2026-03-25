use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use super::styles;

/// Write the Notes tab (備考)
pub fn write_notes(sheet: &mut Worksheet) -> Result<()> {
    sheet.set_name("Notes")?;

    let header = styles::header_format();
    let cell = styles::cell_format();

    // Column widths
    sheet.set_column_width(0, 5)?;
    sheet.set_column_width(1, 25)?;
    sheet.set_column_width(2, 80)?;

    // Headers
    let headers = ["No.", "項目 / Item", "内容 / Content"];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    // Pre-fill some empty rows
    for r in 1..=20 {
        sheet.write_string_with_format(r, 0, "", &cell)?;
        sheet.write_string_with_format(r, 1, "", &cell)?;
        sheet.write_string_with_format(r, 2, "", &cell)?;
    }

    Ok(())
}
