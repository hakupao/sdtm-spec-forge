use indexmap::IndexMap;

/// A node in the domain hierarchy tree
/// Represents a classification level (CAT -> SCAT -> TRT/DECOD/TESTCD)
#[derive(Debug, Clone)]
pub struct TreeNode {
    pub field_name: String,   // e.g., "RSTESTCD", "RSCAT", "RSSCAT"
    pub value: String,        // e.g., "CLINRESP", "CLINICAL UICCV8"
    pub definition: String,   // EDC Japanese description
    pub record_count: usize,
    pub children: IndexMap<String, TreeNode>, // key = "FIELD=VALUE"
}

impl TreeNode {
    pub fn new(field_name: &str, value: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            value: value.to_string(),
            definition: String::new(),
            record_count: 0,
            children: IndexMap::new(),
        }
    }
}

/// The full tree structure for a domain
#[derive(Debug, Clone)]
pub struct DomainTree {
    pub domain: String,
    pub hierarchy_fields: Vec<String>, // e.g., ["RSCAT", "RSSCAT", "RSTESTCD"]
    pub roots: IndexMap<String, TreeNode>,
}

impl DomainTree {
    pub fn new(domain: &str, hierarchy_fields: Vec<String>) -> Self {
        Self {
            domain: domain.to_string(),
            hierarchy_fields,
            roots: IndexMap::new(),
        }
    }

    /// Flatten the tree into table rows for the table view tab
    pub fn flatten(&self) -> Vec<FlatTreeRow> {
        let mut rows = Vec::new();
        for root in self.roots.values() {
            self.flatten_node(root, &mut Vec::new(), &mut rows);
        }
        rows
    }

    fn flatten_node(
        &self,
        node: &TreeNode,
        path: &mut Vec<(String, String, String)>, // (field, value, definition)
        rows: &mut Vec<FlatTreeRow>,
    ) {
        path.push((
            node.field_name.clone(),
            node.value.clone(),
            node.definition.clone(),
        ));

        if node.children.is_empty() {
            // Leaf node: emit a row
            let row = FlatTreeRow {
                domain: self.domain.clone(),
                levels: path.clone(),
                record_count: node.record_count,
            };
            // Propagate record count from leaf
            rows.push(row);
        } else {
            for child in node.children.values() {
                self.flatten_node(child, path, rows);
            }
        }

        path.pop();
    }
}

/// A flattened row from the tree (for table tab)
#[derive(Debug, Clone)]
pub struct FlatTreeRow {
    pub domain: String,
    pub levels: Vec<(String, String, String)>, // (field_name, value, definition)
    pub record_count: usize,
}
