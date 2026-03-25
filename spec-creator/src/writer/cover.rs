use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use super::styles;

/// Write the Cover Page (表紙)
pub fn write_cover(
    sheet: &mut Worksheet,
    study_id: &str,
    date: &str,
    domain_count: usize,
) -> Result<()> {
    sheet.set_name("Cover")?;

    // Column widths
    sheet.set_column_width(0, 5)?;
    sheet.set_column_width(1, 25)?;
    sheet.set_column_width(2, 50)?;
    sheet.set_column_width(3, 5)?;

    // Title
    sheet.merge_range(3, 1, 3, 2, "SDTM Dataset Specification", &styles::title_format())?;

    // Study info with borders
    let labels = [
        (6, "Study ID:", study_id),
        (7, "SDTM Version:", "SDTMIG v3.4"),
        (8, "作成日:", date),
        (9, "ドメイン数:", &domain_count.to_string()),
    ];

    let label_fmt = styles::info_label_bordered_format();
    let value_fmt = styles::info_value_bordered_format();

    for (row, label, value) in labels {
        sheet.write_string_with_format(row, 1, label, &label_fmt)?;
        sheet.write_string_with_format(row, 2, value, &value_fmt)?;
    }

    // Description (3 separate rows, no merge)
    let desc_fmt = styles::info_value_format();
    sheet.write_string_with_format(12, 1, "本ドキュメントは、SDTMデータセットの構造、変数定義、", &desc_fmt)?;
    sheet.write_string_with_format(13, 1, "コードリスト、およびデータの分類体系を記載した仕様書です。", &desc_fmt)?;
    sheet.write_string_with_format(14, 1, "データの理解と検証にご活用ください。", &desc_fmt)?;

    Ok(())
}
