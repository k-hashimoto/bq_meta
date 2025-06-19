use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    pub table: TableInfo,
    pub columns: Vec<ColumnInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub project_id: String,
    pub dataset_id: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub name: String,
    #[serde(rename = "type")]
    pub column_type: String,
    pub description: Option<String>,
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_project: Option<String>,
    pub default_dataset: Option<String>,
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_project: None,
            default_dataset: None,
            output_format: OutputFormat::Table,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub table_path: String,
    pub match_type: MatchType,
    pub matched_content: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone)]
pub enum MatchType {
    TableName,
    TableDescription,
    ColumnName,
    ColumnDescription,
}

impl std::fmt::Display for MatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchType::TableName => write!(f, "TABLE"),
            MatchType::TableDescription => write!(f, "DESC"),
            MatchType::ColumnName => write!(f, "COL"),
            MatchType::ColumnDescription => write!(f, "COL-DESC"),
        }
    }
}