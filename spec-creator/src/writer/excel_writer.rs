use std::path::Path;
use anyhow::Result;
use rust_xlsxwriter::Workbook;
use crate::model::domain::{CodelistMatch, DomainData, MappingRow};

use super::cover;
use super::overview;
use super::change_history;
use super::dataset_overview;
use super::tree_view;
use super::tree_table;
use super::data_trace;
use super::variable_list;
use super::codelist;
use super::notes;

/// Generate the complete specification Excel workbook
pub fn generate_spec(
    output_path: &Path,
    study_id: &str,
    date: &str,
    domains: &[DomainData],
    mappings: &[MappingRow],
    codelist_matches: &[CodelistMatch],
) -> Result<()> {
    let mut workbook = Workbook::new();

    let domain_names: Vec<String> = domains.iter().map(|d| d.name.clone()).collect();

    // Collect all sheets, then hide gridlines on each

    // Tab 1: Cover
    let sheet = workbook.add_worksheet();
    cover::write_cover(sheet, study_id, date, domains.len())?;
    sheet.set_screen_gridlines(false);

    // Tab 2: Overview
    let sheet = workbook.add_worksheet();
    overview::write_overview(sheet, &domain_names)?;
    sheet.set_screen_gridlines(false);

    // Tab 3: Change History
    let sheet = workbook.add_worksheet();
    change_history::write_change_history(sheet, date)?;
    sheet.set_screen_gridlines(false);

    // Tab 4: Dataset List
    let sheet = workbook.add_worksheet();
    dataset_overview::write_dataset_list(sheet, domains)?;
    sheet.set_screen_gridlines(false);

    // Tab 5: Domain Tree View
    let sheet = workbook.add_worksheet();
    tree_view::write_tree_view(sheet, domains)?;
    sheet.set_screen_gridlines(false);

    // Tab 6: Domain Table View
    let sheet = workbook.add_worksheet();
    tree_table::write_tree_table(sheet, domains)?;
    sheet.set_screen_gridlines(false);

    // Tab 7: Data Traceability
    let sheet = workbook.add_worksheet();
    data_trace::write_data_trace(sheet, mappings)?;
    sheet.set_screen_gridlines(false);

    // Tabs 8~N: Variable Lists (one per domain)
    for domain in domains {
        let sheet = workbook.add_worksheet();
        variable_list::write_variable_list(sheet, domain)?;
        sheet.set_screen_gridlines(false);
    }

    // Codelist tab
    let sheet = workbook.add_worksheet();
    codelist::write_codelist(sheet, codelist_matches)?;
    sheet.set_screen_gridlines(false);

    // Notes tab
    let sheet = workbook.add_worksheet();
    notes::write_notes(sheet)?;
    sheet.set_screen_gridlines(false);

    // Save
    workbook.save(output_path)?;

    Ok(())
}
