use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use crate::model::domain::DomainData;
use super::styles;

/// Write the Dataset List tab (データセット一覧)
pub fn write_dataset_list(
    sheet: &mut Worksheet,
    domains: &[DomainData],
) -> Result<()> {
    sheet.set_name("DatasetList")?;

    let header = styles::header_format();
    let cell = styles::cell_format();

    // Column widths (removed データ概要説明)
    let widths = [5.0, 8.0, 45.0, 60.0, 40.0, 15.0, 12.0, 30.0, 30.0];
    for (i, w) in widths.iter().enumerate() {
        sheet.set_column_width(i as u16, *w)?;
    }

    // Headers (removed データ概要説明, renamed Keys to Sort Key)
    let headers = [
        "No.", "Domain", "Dataset Name",
        "Structure", "Sort Key", "変数数\n（使用中）", "レコード数",
        "分類階層", "備考",
    ];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    // Data rows
    for (i, domain) in domains.iter().enumerate() {
        let row = (i + 1) as u32;

        sheet.write_number_with_format(row, 0, (i + 1) as f64, &cell)?;
        sheet.write_string_with_format(row, 1, &domain.name, &cell)?;
        sheet.write_string_with_format(row, 2, &domain.dataset_label, &cell)?;
        sheet.write_string_with_format(row, 3, &domain.structure, &cell)?;
        sheet.write_string_with_format(row, 4, &domain.sort_keys, &cell)?;
        sheet.write_number_with_format(row, 5, domain.variables.len() as f64, &cell)?;
        sheet.write_number_with_format(row, 6, domain.record_count as f64, &cell)?;
        sheet.write_string_with_format(row, 7, &domain.hierarchy_description, &cell)?;
        sheet.write_string_with_format(row, 8, "", &cell)?; // Manual fill
    }

    Ok(())
}
