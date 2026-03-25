use std::collections::HashMap;
use crate::model::variable::{SdtmigVariable, VariableInfo};
use crate::model::domain::{DomainData, MappingRow};
use crate::model::variable::SdtmigDataset;
use crate::reader::csv_reader::CsvData;
use crate::model::tree::DomainTree;

/// Analyze a domain's CSV data against SDTMIG metadata
pub fn analyze_domain(
    csv_data: &CsvData,
    sdtmig_vars: &[SdtmigVariable],
    sdtmig_dataset: Option<&SdtmigDataset>,
    mappings: &[MappingRow],
    sort_keys: &str,
    tree: Option<DomainTree>,
) -> DomainData {
    let domain = &csv_data.domain;

    // Build SDTMIG lookup for this domain
    let ig_lookup: HashMap<&str, &SdtmigVariable> = sdtmig_vars
        .iter()
        .filter(|v| v.domain == *domain)
        .map(|v| (v.name.as_str(), v))
        .collect();

    // Build definition lookup per variable
    let def_lookup = build_var_definitions(mappings, domain);

    // Analyze each variable
    let mut variables = Vec::new();
    let all_count = csv_data.headers.len();

    for (_col_idx, col_name) in csv_data.headers.iter().enumerate() {
        let is_empty = csv_data.is_column_empty(col_name);
        if is_empty {
            continue; // Skip empty columns
        }

        let ig_var = ig_lookup.get(col_name.as_str()).copied();
        let is_custom = ig_var.is_none();

        let unique_vals = csv_data.unique_values(col_name);
        let sample: Vec<String> = unique_vals.iter().take(10).cloned().collect();

        // Determine type: if all non-empty values are numeric, it's Num
        let var_type = infer_type(&csv_data.column_values(col_name));

        let definition = def_lookup
            .get(col_name.as_str())
            .cloned()
            .unwrap_or_default();

        let var_info = VariableInfo {
            name: col_name.clone(),
            label: ig_var.map(|v| v.label.clone()).unwrap_or_default(),
            var_type,
            length: csv_data.max_length(col_name),
            core: if is_custom {
                "Custom".to_string()
            } else {
                ig_var.map(|v| v.core.clone()).unwrap_or("Perm".to_string())
            },
            origin: ig_var.map(|v| v.origin.clone()).unwrap_or_default(),
            role: ig_var.map(|v| v.role.clone()).unwrap_or_default(),
            codelist: ig_var.map(|v| v.codelist_code.clone()).unwrap_or_default(),
            cdisc_notes: ig_var.map(|v| v.cdisc_notes.clone()).unwrap_or_default(),
            definition,
            sample_values: sample,
            is_custom,
            is_empty: false,
            order: ig_var.map(|v| v.order).unwrap_or(9999),
        };

        variables.push(var_info);
    }

    // Sort: standard variables by SDTMIG order, custom at end
    variables.sort_by_key(|v| (v.is_custom, v.order));

    // Build hierarchy description
    let hierarchy_desc = tree
        .as_ref()
        .map(|t| {
            t.hierarchy_fields
                .iter()
                .map(|f| {
                    // Remove domain prefix for display
                    f.strip_prefix(&domain.to_uppercase())
                        .unwrap_or(f)
                        .to_string()
                })
                .collect::<Vec<_>>()
                .join(" → ")
        })
        .unwrap_or_default();

    DomainData {
        name: domain.clone(),
        dataset_label: sdtmig_dataset
            .map(|d| d.label.clone())
            .unwrap_or_default(),
        class: sdtmig_dataset
            .map(|d| d.class.clone())
            .unwrap_or_default(),
        structure: sdtmig_dataset
            .map(|d| d.structure.clone())
            .unwrap_or_default(),
        sort_keys: sort_keys.to_string(),
        record_count: csv_data.record_count(),
        variables,
        all_variable_count: all_count,
        tree,
        hierarchy_description: hierarchy_desc,
    }
}

fn infer_type(values: &[&str]) -> String {
    let is_num = values.iter().all(|v| {
        let trimmed = v.trim();
        trimmed.is_empty() || trimmed.parse::<f64>().is_ok()
    });
    if is_num { "Num".to_string() } else { "Char".to_string() }
}

/// Build a combined definition string for each variable in the domain
fn build_var_definitions(mappings: &[MappingRow], domain: &str) -> HashMap<String, String> {
    let mut var_defs: HashMap<String, Vec<String>> = HashMap::new();

    for m in mappings {
        if m.domain.eq_ignore_ascii_case(domain) && !m.definition.is_empty() {
            var_defs
                .entry(m.variable.clone())
                .or_default()
                .push(m.definition.clone());
        }
    }

    // Deduplicate and join
    var_defs
        .into_iter()
        .map(|(var, mut defs)| {
            defs.dedup();
            let combined = if defs.len() <= 3 {
                defs.join("; ")
            } else {
                format!("{}; ... ({}件)", defs[..3].join("; "), defs.len())
            };
            (var, combined)
        })
        .collect()
}
