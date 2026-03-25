use crate::model::variable::VariableInfo;
use crate::model::tree::DomainTree;

/// Complete analyzed data for one SDTM domain
#[derive(Debug, Clone)]
pub struct DomainData {
    pub name: String,                    // e.g., "RS"
    pub dataset_label: String,           // e.g., "Disease Response and Clin Classification"
    pub class: String,                   // e.g., "Findings"
    pub structure: String,               // e.g., "One record per ..."
    pub sort_keys: String,               // from DomainsSetting
    pub record_count: usize,
    pub variables: Vec<VariableInfo>,    // only non-empty variables
    pub all_variable_count: usize,       // total CSV columns
    pub tree: Option<DomainTree>,
    pub hierarchy_description: String,   // e.g., "TESTCD → CAT → SCAT"
}

/// Mapping row from OperationConf.xlsx
#[derive(Debug, Clone)]
pub struct MappingRow {
    pub definition: String,
    pub domain: String,
    pub variable: String,
    pub terminology: String,
    pub filename: String,
    pub fieldname: String,
    pub label: String,
    pub oper_type: String,
    pub parameter: String,
    pub notes: String,
}

/// CDISC Terminology entry
#[derive(Debug, Clone)]
pub struct TerminologyEntry {
    pub code: String,
    pub codelist_code: String,
    pub codelist_name: String,
    pub submission_value: String,
    pub cdisc_definition: String,
}

/// Codelist match result for output
#[derive(Debug, Clone)]
pub struct CodelistMatch {
    pub domain: String,
    pub variable: String,
    pub codelist_code: String,
    pub codelist_name: String,
    pub actual_value: String,
    pub cdisc_submission_value: String,
    pub cdisc_definition: String,
    pub is_match: bool,
}
