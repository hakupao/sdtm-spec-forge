use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use super::styles;

/// Sheet descriptions for the Overview tab
struct SheetDesc {
    name: &'static str,
    description_jp: &'static str,
}

const FIXED_SHEETS: &[SheetDesc] = &[
    SheetDesc {
        name: "Cover",
        description_jp: "表紙。Study ID、SDTM版本、作成日等の基本情報を記載。",
    },
    SheetDesc {
        name: "Overview",
        description_jp: "本シート。ドキュメント全体の構成と各シートの説明を記載。",
    },
    SheetDesc {
        name: "ChangeHistory",
        description_jp: "更新履歴。ドキュメントの変更記録。",
    },
    SheetDesc {
        name: "DatasetList",
        description_jp: "データセット一覧。全ドメインの概要（ドメイン名、レコード数、構造、分類階層等）。",
    },
    SheetDesc {
        name: "DomainTree",
        description_jp: "ドメイン構造概覧（ツリー形式）。各ドメイン内のCAT/SCAT分類を樹形図で表示。EDCの原始日本語説明を付記。",
    },
    SheetDesc {
        name: "DomainTable",
        description_jp: "ドメイン構造概覧（テーブル形式）。ツリー構造をテーブル形式で展開し、各分類パスのEDC説明とレコード数を表示。",
    },
    SheetDesc {
        name: "DataTrace",
        description_jp: "データ来源追跡。EDC原始データフィールドからSDTM変数へのマッピング対応関係を表示。",
    },
];

/// Write the Overview tab
pub fn write_overview(
    sheet: &mut Worksheet,
    domain_names: &[String],
) -> Result<()> {
    sheet.set_name("Overview")?;

    // Column widths
    sheet.set_column_width(0, 5)?;
    sheet.set_column_width(1, 20)?;
    sheet.set_column_width(2, 60)?;

    // Title
    let header = styles::header_format();
    let cell = styles::cell_format();

    sheet.write_string_with_format(0, 0, "No.", &header)?;
    sheet.write_string_with_format(0, 1, "Sheet Name", &header)?;
    sheet.write_string_with_format(0, 2, "内容説明", &header)?;
    sheet.set_freeze_panes(1, 0)?;

    let mut row = 1u32;

    // Fixed sheets
    for (i, desc) in FIXED_SHEETS.iter().enumerate() {
        sheet.write_number_with_format(row, 0, (i + 1) as f64, &cell)?;
        sheet.write_string_with_format(row, 1, desc.name, &cell)?;
        sheet.write_string_with_format(row, 2, desc.description_jp, &cell)?;
        row += 1;
    }

    // Domain variable list tabs
    for (i, domain) in domain_names.iter().enumerate() {
        let no = FIXED_SHEETS.len() + i + 1;
        let desc_jp = format!("{}ドメインの変数一覧。使用中の変数のメタデータ、データ型等を記載。", domain);

        sheet.write_number_with_format(row, 0, no as f64, &cell)?;
        sheet.write_string_with_format(row, 1, domain, &cell)?;
        sheet.write_string_with_format(row, 2, &desc_jp, &cell)?;
        row += 1;
    }

    // Codelist tab
    let no = FIXED_SHEETS.len() + domain_names.len() + 1;
    sheet.write_number_with_format(row, 0, no as f64, &cell)?;
    sheet.write_string_with_format(row, 1, "Codelist", &cell)?;
    sheet.write_string_with_format(row, 2, "コードリスト一覧。データ中の実際値とCDISC CT標準Termの対照表。", &cell)?;
    row += 1;

    // Notes tab
    let no = no + 1;
    sheet.write_number_with_format(row, 0, no as f64, &cell)?;
    sheet.write_string_with_format(row, 1, "Notes", &cell)?;
    sheet.write_string_with_format(row, 2, "備考。特殊処理、手動補充用の自由記入欄。", &cell)?;

    Ok(())
}
