/// SDTM variable metadata combining SDTMIG spec + actual data analysis
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub name: String,
    pub label: String,
    pub var_type: String,       // "Char" or "Num"
    pub length: usize,          // max actual length from data
    pub core: String,           // "Req", "Exp", "Perm", "Custom"
    pub origin: String,
    pub role: String,
    pub codelist: String,       // CDISC CT codelist name
    pub cdisc_notes: String,
    pub definition: String,     // EDC Japanese description from Mapping DEFINITION
    pub sample_values: Vec<String>,
    pub is_custom: bool,        // true if not in SDTMIG
    pub is_empty: bool,         // true if all values empty in data
    pub order: usize,           // variable order from SDTMIG
}

/// SDTMIG variable spec (from SDTMIG_v3.4.xlsx Variables sheet)
#[derive(Debug, Clone)]
pub struct SdtmigVariable {
    pub domain: String,
    pub name: String,
    pub label: String,
    pub var_type: String,
    pub core: String,
    pub origin: String,
    pub role: String,
    pub codelist_code: String,
    pub cdisc_notes: String,
    pub order: usize,
}

/// SDTMIG dataset spec (from SDTMIG_v3.4.xlsx Datasets sheet)
#[derive(Debug, Clone)]
pub struct SdtmigDataset {
    pub domain: String,
    pub label: String,
    pub class: String,
    pub structure: String,
}
