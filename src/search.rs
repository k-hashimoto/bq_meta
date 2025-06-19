use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

use crate::models::{SearchResult, MatchType};
use crate::storage::{list_tables, load_table_metadata};

pub struct SearchOptions {
    pub pattern: String,
    pub regex: bool,
    pub case_sensitive: bool,
    pub search_all: bool,
    pub search_table_desc: bool,
    pub search_column_name: bool,
    pub search_column_desc: bool,
    pub project_filter: Option<String>,
    pub dataset_filter: Option<String>,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            pattern: String::new(),
            regex: false,
            case_sensitive: false,
            search_all: false,
            search_table_desc: false,
            search_column_name: false,
            search_column_desc: false,
            project_filter: None,
            dataset_filter: None,
        }
    }
}

pub fn search_tables(options: &SearchOptions) -> Result<Vec<SearchResult>> {
    let tables = list_tables(
        options.project_filter.as_deref(),
        options.dataset_filter.as_deref(),
    )?;

    let mut results = Vec::new();
    let regex = if options.regex {
        Some(if options.case_sensitive {
            Regex::new(&options.pattern)?
        } else {
            Regex::new(&format!("(?i){}", options.pattern))?
        })
    } else {
        None
    };

    for (project_id, dataset_id, table_name) in tables {
        let table_path = format!("{}.{}.{}", project_id, dataset_id, table_name);
        
        // Load table metadata
        let metadata = match load_table_metadata(&project_id, &dataset_id, &table_name) {
            Ok(m) => m,
            Err(_) => continue, // Skip if we can't load metadata
        };

        // Search table name (default behavior)
        if !options.search_all && !options.search_table_desc && !options.search_column_name && !options.search_column_desc {
            if matches_pattern(&table_name, &options.pattern, &regex, options.case_sensitive) {
                results.push(SearchResult {
                    table_path: table_path.clone(),
                    match_type: MatchType::TableName,
                    matched_content: table_name.clone(),
                    context: None,
                });
            }
        } else {
            // Search based on specific options
            if options.search_all || options.search_table_desc {
                // Search table name
                if matches_pattern(&table_name, &options.pattern, &regex, options.case_sensitive) {
                    results.push(SearchResult {
                        table_path: table_path.clone(),
                        match_type: MatchType::TableName,
                        matched_content: table_name.clone(),
                        context: None,
                    });
                }

                // Search table description
                if let Some(ref desc) = metadata.table.description {
                    if matches_pattern(desc, &options.pattern, &regex, options.case_sensitive) {
                        results.push(SearchResult {
                            table_path: table_path.clone(),
                            match_type: MatchType::TableDescription,
                            matched_content: desc.clone(),
                            context: None,
                        });
                    }
                }
            }

            if options.search_all || options.search_column_name || options.search_column_desc {
                // Search columns
                for column in &metadata.columns {
                    // Search column name
                    if options.search_all || options.search_column_name {
                        if matches_pattern(&column.name, &options.pattern, &regex, options.case_sensitive) {
                            results.push(SearchResult {
                                table_path: table_path.clone(),
                                match_type: MatchType::ColumnName,
                                matched_content: column.name.clone(),
                                context: Some(format!("Column: {}", column.name)),
                            });
                        }
                    }

                    // Search column description
                    if options.search_all || options.search_column_desc {
                        if let Some(ref desc) = column.description {
                            if matches_pattern(desc, &options.pattern, &regex, options.case_sensitive) {
                                results.push(SearchResult {
                                    table_path: table_path.clone(),
                                    match_type: MatchType::ColumnDescription,
                                    matched_content: desc.clone(),
                                    context: Some(format!("Column: {}", column.name)),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // Remove duplicates and sort
    let mut unique_results: HashMap<String, SearchResult> = HashMap::new();
    for result in results {
        let key = format!("{}:{}:{}", result.table_path, result.match_type, result.matched_content);
        unique_results.entry(key).or_insert(result);
    }

    let mut final_results: Vec<SearchResult> = unique_results.into_values().collect();
    final_results.sort_by(|a, b| a.table_path.cmp(&b.table_path));

    Ok(final_results)
}

fn matches_pattern(text: &str, pattern: &str, regex: &Option<Regex>, case_sensitive: bool) -> bool {
    if let Some(ref re) = regex {
        re.is_match(text)
    } else if case_sensitive {
        text.contains(pattern)
    } else {
        text.to_lowercase().contains(&pattern.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matches_pattern() {
        // Case insensitive substring match
        assert!(matches_pattern("user_events", "user", &None, false));
        assert!(matches_pattern("user_events", "USER", &None, false));
        assert!(!matches_pattern("events", "user", &None, false));

        // Case sensitive substring match
        assert!(matches_pattern("user_events", "user", &None, true));
        assert!(!matches_pattern("user_events", "USER", &None, true));

        // Regex match
        let regex = Regex::new(r"^user_.*").unwrap();
        assert!(matches_pattern("user_events", "", &Some(regex), false));
        
        let regex = Regex::new(r"events$").unwrap();
        assert!(matches_pattern("user_events", "", &Some(regex), false));
        assert!(!matches_pattern("user_data", "", &Some(regex), false));
    }
}