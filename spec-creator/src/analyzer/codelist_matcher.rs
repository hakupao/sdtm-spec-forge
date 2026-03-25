use std::collections::HashMap;
use crate::model::domain::{CodelistMatch, DomainData, TerminologyEntry};

/// Match actual data values against CDISC Terminology
pub fn match_codelists(
    domains: &[DomainData],
    terminology: &[TerminologyEntry],
) -> Vec<CodelistMatch> {
    // Build terminology lookup: codelist_code -> [entries]
    let mut ct_by_code: HashMap<String, Vec<&TerminologyEntry>> = HashMap::new();
    for entry in terminology {
        if !entry.codelist_code.is_empty() {
            ct_by_code
                .entry(entry.codelist_code.clone())
                .or_default()
                .push(entry);
        }
    }

    // Also build codelist_code -> codelist_name lookup
    let mut code_to_name: HashMap<String, String> = HashMap::new();
    for entry in terminology {
        if entry.codelist_code.is_empty() && !entry.code.is_empty() {
            // This is a header row for the codelist
            code_to_name.insert(entry.code.clone(), entry.codelist_name.clone());
        }
    }

    let mut matches = Vec::new();

    for domain in domains {
        for var in &domain.variables {
            if var.codelist.is_empty() || var.sample_values.is_empty() {
                continue;
            }

            // The codelist field may contain multiple codes separated by ";"
            let codes: Vec<&str> = var.codelist.split(';').map(|s| s.trim()).collect();

            for code in &codes {
                let codelist_name = code_to_name
                    .get(*code)
                    .cloned()
                    .unwrap_or_else(|| code.to_string());

                let ct_entries = ct_by_code.get(*code);

                // Build submission value set for matching
                let ct_values: HashMap<String, &TerminologyEntry> = ct_entries
                    .map(|entries| {
                        entries
                            .iter()
                            .map(|e| (e.submission_value.to_uppercase(), *e))
                            .collect()
                    })
                    .unwrap_or_default();

                for actual_val in &var.sample_values {
                    let upper = actual_val.to_uppercase();
                    let matched = ct_values.get(&upper);

                    matches.push(CodelistMatch {
                        domain: domain.name.clone(),
                        variable: var.name.clone(),
                        codelist_code: code.to_string(),
                        codelist_name: codelist_name.clone(),
                        actual_value: actual_val.clone(),
                        cdisc_submission_value: matched
                            .map(|e| e.submission_value.clone())
                            .unwrap_or_default(),
                        cdisc_definition: matched
                            .map(|e| e.cdisc_definition.clone())
                            .unwrap_or_default(),
                        is_match: matched.is_some(),
                    });
                }
            }
        }
    }

    matches
}
