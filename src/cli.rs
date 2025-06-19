use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bq-meta")]
#[command(about = "BigQuery table metadata management CLI tool")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize metadata directory
    Init {
        /// Custom path for metadata directory
        #[arg(short, long)]
        path: Option<String>,
    },
    /// List tables
    List {
        /// Project ID to filter
        #[arg(short, long)]
        project: Option<String>,
        /// Dataset ID to filter
        #[arg(short, long)]  
        dataset: Option<String>,
        /// Output format
        #[arg(short, long, default_value = "table")]
        output: String,
    },
    /// Search tables
    Search {
        /// Search pattern
        pattern: String,
        /// Search all fields (table name, description, columns)
        #[arg(long)]
        all: bool,
        /// Search only table descriptions
        #[arg(long)]
        desc: bool,
        /// Search only column names
        #[arg(long)]
        column: bool,
        /// Search only column descriptions
        #[arg(long, name = "col-desc")]
        col_desc: bool,
        /// Use regular expressions
        #[arg(long)]
        regex: bool,
        /// Case sensitive search
        #[arg(long)]
        case_sensitive: bool,
        /// Project ID to filter
        #[arg(short, long)]
        project: Option<String>,
        /// Dataset ID to filter
        #[arg(short, long)]
        dataset: Option<String>,
        /// Output format
        #[arg(short, long, default_value = "table")]
        output: String,
    },
    /// Show table details
    Show {
        /// Table specification (project.dataset.table)
        table: String,
        /// Output format
        #[arg(short, long, default_value = "table")]
        output: String,
    },
    /// Describe table columns
    Describe {
        /// Table specification (project.dataset.table)
        table: String,
        /// Output format
        #[arg(short, long, default_value = "table")]
        output: String,
    },
    /// Create new table metadata
    Create {
        /// Table specification (project.dataset.table)
        table: String,
        /// Table description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Edit column description
    Edit {
        /// Table specification (project.dataset.table)
        table: String,
        /// Column name
        column: String,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Export table metadata
    Export {
        /// Table specification (project.dataset.table)
        table: String,
        /// Output file path
        #[arg(short, long)]
        file: Option<String>,
        /// Output format
        #[arg(short = 'f', long, default_value = "yaml")]
        format: String,
    },
    /// Import table metadata
    Import {
        /// Input file path
        file: String,
        /// Force overwrite existing metadata
        #[arg(long)]
        force: bool,
    },
    /// Manage configuration
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
pub enum ConfigAction {
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// List all configuration
    List,
}

pub fn parse_table_spec(table_spec: &str) -> Result<(String, String, String), String> {
    let parts: Vec<&str> = table_spec.split('.').collect();
    if parts.len() != 3 {
        return Err("Table specification must be in format: project.dataset.table".to_string());
    }
    Ok((parts[0].to_string(), parts[1].to_string(), parts[2].to_string()))
}