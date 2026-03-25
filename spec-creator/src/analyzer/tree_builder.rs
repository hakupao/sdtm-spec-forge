use std::collections::HashMap;
use indexmap::IndexMap;
use crate::model::tree::{DomainTree, TreeNode};
use crate::model::domain::MappingRow;
use crate::reader::csv_reader::CsvData;

/// Detect hierarchy fields for a domain based on its class and available columns
/// Hierarchy order: CAT → SCAT → (TRT | DECOD | CAT1 → TESTCD)
fn detect_hierarchy_fields(domain: &str, headers: &[String]) -> Vec<String> {
    let prefix = domain.to_uppercase();
    let has = |suffix: &str| -> bool {
        let col = format!("{}{}", prefix, suffix);
        headers.iter().any(|h| h == &col)
    };

    let mut fields = Vec::new();

    // Root level is always CAT if it exists
    if has("CAT") {
        fields.push(format!("{}CAT", prefix));
    }

    // Second level: SCAT
    if has("SCAT") {
        fields.push(format!("{}SCAT", prefix));
    }

    // Leaf level: TRT (Interventions), DECOD (Events), or TESTCD (Findings)
    if has("TRT") {
        fields.push(format!("{}TRT", prefix));
    } else if has("DECOD") {
        fields.push(format!("{}DECOD", prefix));
    } else if has("TESTCD") {
        fields.push(format!("{}TESTCD", prefix));
    }

    fields
}

/// Build tree structure for a domain from actual data
pub fn build_domain_tree(
    csv_data: &CsvData,
    mappings: &[MappingRow],
) -> Option<DomainTree> {
    let hierarchy = detect_hierarchy_fields(&csv_data.domain, &csv_data.headers);
    if hierarchy.is_empty() {
        return None;
    }

    // Filter to only non-empty hierarchy fields
    let active_hierarchy: Vec<String> = hierarchy
        .iter()
        .filter(|h| !csv_data.is_column_empty(h))
        .cloned()
        .collect();

    if active_hierarchy.is_empty() {
        return None;
    }

    let mut tree = DomainTree::new(&csv_data.domain, active_hierarchy.clone());

    // Get column indices
    let col_indices: Vec<Option<usize>> = active_hierarchy
        .iter()
        .map(|h| csv_data.headers.iter().position(|c| c == h))
        .collect();

    // Process each row - build tree without definitions first
    for row in &csv_data.rows {
        let values: Vec<String> = col_indices
            .iter()
            .map(|idx| {
                idx.map(|i| row.get(i).map(|v| v.trim().to_string()).unwrap_or_default())
                    .unwrap_or_default()
            })
            .collect();

        // Skip if the root level is empty
        if values[0].is_empty() {
            continue;
        }

        insert_into_tree(&mut tree.roots, &active_hierarchy, &values, 0);
    }

    // Build definition lookup keyed by full hierarchy path, then assign to leaf nodes only
    let definition_lookup = build_definition_lookup(mappings, &csv_data.domain, &active_hierarchy);
    assign_leaf_definitions(&mut tree.roots, &mut Vec::new(), &definition_lookup);

    // Calculate record counts
    calculate_counts(&mut tree.roots);

    Some(tree)
}

fn insert_into_tree(
    nodes: &mut IndexMap<String, TreeNode>,
    fields: &[String],
    values: &[String],
    level: usize,
) {
    if level >= fields.len() || values[level].is_empty() {
        return;
    }

    let key = format!("{}={}", fields[level], values[level]);
    let node = nodes.entry(key.clone()).or_insert_with(|| {
        TreeNode::new(&fields[level], &values[level])
    });

    node.record_count += 1;

    if level + 1 < fields.len() && !values[level + 1].is_empty() {
        insert_into_tree(&mut node.children, fields, values, level + 1);
    }
}

/// Assign definitions only to leaf nodes using composite hierarchy path lookup
fn assign_leaf_definitions(
    nodes: &mut IndexMap<String, TreeNode>,
    path: &mut Vec<String>,  // "FIELD=VALUE" segments
    definitions: &HashMap<String, Vec<String>>,
) {
    for node in nodes.values_mut() {
        path.push(format!("{}={}", node.field_name.to_uppercase(), node.value.to_uppercase()));

        if node.children.is_empty() {
            // Leaf node: look up definition by composite path
            let key = path.join("|");
            if let Some(defs) = definitions.get(&key) {
                node.definition = defs.join("\n");
            }
        } else {
            assign_leaf_definitions(&mut node.children, path, definitions);
        }

        path.pop();
    }
}

fn calculate_counts(nodes: &mut IndexMap<String, TreeNode>) {
    for node in nodes.values_mut() {
        if !node.children.is_empty() {
            calculate_counts(&mut node.children);
        }
    }
}

/// Build a lookup from full hierarchy path to definition
/// Key format: "RSCAT=CLINICAL UICCV8|RSSCAT=T_CATEGORY|RSTESTCD=CLINRESP"
fn build_definition_lookup(
    mappings: &[MappingRow],
    domain: &str,
    hierarchy_fields: &[String],
) -> HashMap<String, Vec<String>> {
    let domain_mappings: Vec<&MappingRow> = mappings
        .iter()
        .filter(|m| m.domain.eq_ignore_ascii_case(domain))
        .collect();

    // Group into definition blocks (contiguous rows sharing the same non-empty definition)
    let mut blocks: Vec<(String, Vec<&MappingRow>)> = Vec::new();
    let mut current_def = String::new();

    for m in &domain_mappings {
        if !m.definition.is_empty() && m.definition != current_def {
            current_def = m.definition.clone();
            blocks.push((current_def.clone(), Vec::new()));
        }
        if let Some(block) = blocks.last_mut() {
            block.1.push(m);
        }
    }

    let mut lookup: HashMap<String, Vec<String>> = HashMap::new();

    for (definition, rows) in &blocks {
        // Extract parameter values for each hierarchy field present in this block
        let key_parts: Vec<String> = hierarchy_fields
            .iter()
            .filter_map(|field| {
                rows.iter()
                    .find(|m| m.variable.eq_ignore_ascii_case(field) && !m.parameter.is_empty())
                    .map(|m| format!("{}={}", field.to_uppercase(), m.parameter.to_uppercase()))
            })
            .collect();

        let key = key_parts.join("|");
        if !key.is_empty() {
            lookup.entry(key).or_insert_with(Vec::new).push(definition.clone());
        }
    }

    lookup
}
