use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use super::styles;

/// Write the Change History tab (更新履歴)
pub fn write_change_history(sheet: &mut Worksheet, date: &str) -> Result<()> {
    sheet.set_name("ChangeHistory")?;

    let header = styles::header_format();
    let cell = styles::cell_format();

    // Column widths
    sheet.set_column_width(0, 5)?;
    sheet.set_column_width(1, 15)?;
    sheet.set_column_width(2, 15)?;
    sheet.set_column_width(3, 60)?;
    sheet.set_column_width(4, 20)?;

    // Headers
    let headers = ["No.", "日付 / Date", "版数 / Version", "変更内容 / Description", "担当者 / Author"];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    // Initial entry
    sheet.write_number_with_format(1, 0, 1.0, &cell)?;
    sheet.write_string_with_format(1, 1, date, &cell)?;
    sheet.write_string_with_format(1, 2, "1.0", &cell)?;
    sheet.write_string_with_format(1, 3, "初版作成 / Initial creation", &cell)?;
    sheet.write_string_with_format(1, 4, "", &cell)?;

    // Empty rows for future entries
    for r in 2..=10 {
        for c in 0..5 {
            sheet.write_string_with_format(r, c, "", &cell)?;
        }
    }

    Ok(())
}
