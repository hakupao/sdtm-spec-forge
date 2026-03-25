use rust_xlsxwriter::Worksheet;
use anyhow::Result;
use crate::model::domain::DomainData;
use crate::model::tree::TreeNode;
use super::styles;

/// Write the Domain Tree View tab (ドメイン構造概覧 - ツリー形式)
pub fn write_tree_view(
    sheet: &mut Worksheet,
    domains: &[DomainData],
) -> Result<()> {
    sheet.set_name("DomainTree")?;

    // Column widths
    sheet.set_column_width(0, 80)?;
    sheet.set_column_width(1, 40)?;
    sheet.set_column_width(2, 12)?;

    let header = styles::header_format();
    sheet.write_string_with_format(0, 0, "ドメイン構造 / Domain Structure", &header)?;
    sheet.write_string_with_format(0, 1, "EDC説明 / EDC Description", &header)?;
    sheet.write_string_with_format(0, 2, "レコード数", &header)?;
    sheet.set_freeze_panes(1, 0)?;

    let mut row = 1u32;

    for domain in domains {
        let tree = match &domain.tree {
            Some(t) => t,
            None => continue,
        };

        if tree.roots.is_empty() {
            continue;
        }

        // Domain header
        let section = styles::section_header_format();
        let domain_title = format!(
            "【{} - {}】",
            domain.name, domain.dataset_label
        );
        sheet.merge_range(row, 0, row, 2, &domain_title, &section)?;
        row += 1;

        // Render tree nodes
        for root in tree.roots.values() {
            write_tree_node(sheet, root, 0, &mut row)?;
        }

        // Blank separator row
        row += 1;
    }

    Ok(())
}

fn write_tree_node(
    sheet: &mut Worksheet,
    node: &TreeNode,
    depth: u8,
    row: &mut u32,
) -> Result<()> {
    let indent_fmt = styles::tree_indent_format(depth);
    let cell = styles::cell_format();

    // Build display text with tree characters
    let prefix = if depth == 0 {
        String::new()
    } else {
        "  ".repeat(depth as usize)
    };

    let text = format!("{}{}={}", prefix, node.field_name, node.value);

    sheet.write_string_with_format(*row, 0, &text, &indent_fmt)?;
    sheet.write_string_with_format(*row, 1, &node.definition, &cell)?;

    if node.children.is_empty() {
        sheet.write_number_with_format(*row, 2, node.record_count as f64, &cell)?;
    } else {
        sheet.write_string_with_format(*row, 2, "", &cell)?;
    }

    *row += 1;

    for child in node.children.values() {
        write_tree_node(sheet, child, depth + 1, row)?;
    }

    Ok(())
}
