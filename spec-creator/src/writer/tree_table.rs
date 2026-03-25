use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use crate::model::domain::DomainData;
use super::styles;

/// Write the Domain Table View tab (ドメイン構造概覧 - テーブル形式)
pub fn write_tree_table(
    sheet: &mut Worksheet,
    domains: &[DomainData],
) -> Result<()> {
    sheet.set_name("DomainTable")?;

    let header = styles::header_format();
    let cell = styles::cell_format();

    // Column widths (Domain, CAT, SCAT, TRT/DECOD/TESTCD, EDC説明, レコード数)
    let widths = [8.0, 30.0, 30.0, 30.0, 40.0, 12.0];
    for (i, w) in widths.iter().enumerate() {
        sheet.set_column_width(i as u16, *w)?;
    }

    // Headers
    let headers = [
        "Domain", "CAT", "SCAT", "TRT/DECOD/TESTCD",
        "EDC説明", "レコード数",
    ];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header)?;
    }
    sheet.set_freeze_panes(1, 0)?;

    let mut row = 1u32;

    for domain in domains {
        let tree = match &domain.tree {
            Some(t) => t,
            None => continue,
        };

        let flat_rows = tree.flatten();

        for flat_row in &flat_rows {
            sheet.write_string_with_format(row, 0, &flat_row.domain, &cell)?;

            // Map hierarchy levels to columns: CAT(1), SCAT(2), Leaf(3)
            // The hierarchy is always: CAT → SCAT → TRT/DECOD/TESTCD
            // Some domains may skip levels (e.g., MI only has TESTCD)
            let mut cat_val = String::new();
            let mut scat_val = String::new();
            let mut leaf_val = String::new();

            for (field, value, _definition) in &flat_row.levels {
                // Strip domain prefix then match the SDTM standard suffix
                let suffix = field.strip_prefix(&flat_row.domain).unwrap_or(field);
                if suffix == "SCAT" {
                    scat_val = value.clone();
                } else if suffix == "CAT" {
                    cat_val = value.clone();
                } else {
                    // TRT, DECOD, TESTCD = leaf
                    leaf_val = value.clone();
                }
            }

            // Definition comes from the leaf node only (last level)
            let leaf_definition = flat_row.levels.last()
                .map(|(_, _, def)| def.as_str())
                .unwrap_or("");

            sheet.write_string_with_format(row, 1, &cat_val, &cell)?;
            sheet.write_string_with_format(row, 2, &scat_val, &cell)?;
            sheet.write_string_with_format(row, 3, &leaf_val, &cell)?;
            sheet.write_string_with_format(row, 4, leaf_definition, &cell)?;
            sheet.write_number_with_format(row, 5, flat_row.record_count as f64, &cell)?;
            row += 1;
        }
    }

    Ok(())
}
