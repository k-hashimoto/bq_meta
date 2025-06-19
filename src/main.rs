use anyhow::Result;
use clap::Parser;
use colored::*;
use std::io::{self, Write};

use bq_meta::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Init { path } => {
            if let Some(custom_path) = path {
                std::env::set_var("BQ_META_PATH", custom_path);
            }
            init_data_directory()?;
        }
        Commands::List { project, dataset, output } => {
            let tables = list_tables(project.as_deref(), dataset.as_deref())?;
            display_table_list(&tables, &output)?;
        }
        Commands::Search { 
            pattern, all, desc, column, col_desc, regex, case_sensitive, 
            project, dataset, output 
        } => {
            let options = SearchOptions {
                pattern,
                regex,
                case_sensitive,
                search_all: all,
                search_table_desc: desc,
                search_column_name: column,
                search_column_desc: col_desc,
                project_filter: project,
                dataset_filter: dataset,
            };
            let results = search_tables(&options)?;
            display_search_results(&results, &output)?;
        }
        Commands::Show { table, output } => {
            let (project, dataset, table_name) = parse_table_spec(&table)
                .map_err(|e| anyhow::anyhow!(e))?;
            let metadata = load_table_metadata(&project, &dataset, &table_name)?;
            display_table_metadata(&metadata, &output)?;
        }
        Commands::Describe { table, output } => {
            let (project, dataset, table_name) = parse_table_spec(&table)
                .map_err(|e| anyhow::anyhow!(e))?;
            let metadata = load_table_metadata(&project, &dataset, &table_name)?;
            display_column_descriptions(&metadata, &output)?;
        }
        Commands::Create { table, description } => {
            let (project, dataset, table_name) = parse_table_spec(&table)
                .map_err(|e| anyhow::anyhow!(e))?;
            create_table_metadata(&project, &dataset, &table_name, description)?;
        }
        Commands::Edit { table, column, description } => {
            let (project, dataset, table_name) = parse_table_spec(&table)
                .map_err(|e| anyhow::anyhow!(e))?;
            edit_column_description(&project, &dataset, &table_name, &column, description)?;
        }
        Commands::Export { table, file, format } => {
            let (project, dataset, table_name) = parse_table_spec(&table)
                .map_err(|e| anyhow::anyhow!(e))?;
            export_table_metadata(&project, &dataset, &table_name, file.as_deref(), &format)?;
        }
        Commands::Import { file, force } => {
            import_table_metadata(&file, force)?;
        }
        Commands::Config { action } => {
            match action {
                ConfigAction::Set { key, value } => {
                    set_config_value(&key, &value)?;
                }
                ConfigAction::Get { key } => {
                    get_config_value(&key)?;
                }
                ConfigAction::List => {
                    list_config()?;
                }
            }
        }
    }
    
    Ok(())
}

fn display_table_list(tables: &[(String, String, String)], output_format: &str) -> Result<()> {
    match output_format {
        "json" => {
            let json = serde_json::to_string_pretty(&tables)?;
            println!("{}", json);
        }
        "yaml" => {
            let yaml = serde_yaml::to_string(&tables)?;
            println!("{}", yaml);
        }
        _ => {
            if tables.is_empty() {
                println!("No tables found.");
                return Ok(());
            }
            
            println!("{}", "Project.Dataset.Table".bold());
            println!("{}", "─".repeat(50));
            for (project, dataset, table) in tables {
                println!("{}.{}.{}", project.cyan(), dataset.yellow(), table.green());
            }
        }
    }
    Ok(())
}

fn display_search_results(results: &[SearchResult], output_format: &str) -> Result<()> {
    match output_format {
        "json" => {
            let json_results: Vec<serde_json::Value> = results.iter().map(|r| {
                serde_json::json!({
                    "table_path": r.table_path,
                    "match_type": r.match_type.to_string(),
                    "matched_content": r.matched_content,
                    "context": r.context
                })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&json_results)?);
        }
        _ => {
            if results.is_empty() {
                println!("No matches found.");
                return Ok(());
            }
            
            for result in results {
                let match_type_colored = match result.match_type {
                    MatchType::TableName => format!("[{}]", "TABLE".green()),
                    MatchType::TableDescription => format!("[{}]", "DESC".blue()),
                    MatchType::ColumnName => format!("[{}]", "COL".yellow()),
                    MatchType::ColumnDescription => format!("[{}]", "COL-DESC".magenta()),
                };
                
                print!("{} {}", match_type_colored, result.table_path.cyan());
                if let Some(ref context) = result.context {
                    print!(" ({})", context.dimmed());
                }
                println!();
                println!("  {}", result.matched_content.italic());
                println!();
            }
        }
    }
    Ok(())
}

fn display_table_metadata(metadata: &TableMetadata, output_format: &str) -> Result<()> {
    match output_format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(metadata)?);
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(metadata)?);
        }
        _ => {
            println!("{}", "Table Information".bold());
            println!("{}", "─".repeat(30));
            println!("Name: {}", metadata.table.name.green());
            println!("Project: {}", metadata.table.project_id.cyan());
            println!("Dataset: {}", metadata.table.dataset_id.yellow());
            if let Some(ref desc) = metadata.table.description {
                println!("Description: {}", desc);
            }
            
            println!("\n{}", "Columns".bold());
            println!("{}", "─".repeat(30));
            for column in &metadata.columns {
                println!("  {} ({})", column.name.green(), column.column_type.blue());
                if let Some(ref desc) = column.description {
                    println!("    {}", desc.italic());
                }
            }
        }
    }
    Ok(())
}

fn display_column_descriptions(metadata: &TableMetadata, output_format: &str) -> Result<()> {
    match output_format {
        "json" => {
            let columns_with_desc: Vec<_> = metadata.columns.iter()
                .map(|c| serde_json::json!({
                    "name": c.name,
                    "type": c.column_type,
                    "description": c.description
                })).collect();
            println!("{}", serde_json::to_string_pretty(&columns_with_desc)?);
        }
        "yaml" => {
            let columns_with_desc: Vec<_> = metadata.columns.iter()
                .map(|c| (c.name.clone(), c.column_type.clone(), c.description.clone()))
                .collect();
            println!("{}", serde_yaml::to_string(&columns_with_desc)?);
        }
        _ => {
            println!("{} - Column Descriptions", metadata.table.name.bold());
            println!("{}", "─".repeat(50));
            for column in &metadata.columns {
                println!("{} ({})", column.name.green(), column.column_type.blue());
                if let Some(ref desc) = column.description {
                    println!("  {}", desc.italic());
                } else {
                    println!("  {}", "No description".dimmed());
                }
                println!();
            }
        }
    }
    Ok(())
}

fn create_table_metadata(project: &str, dataset: &str, table_name: &str, description: Option<String>) -> Result<()> {
    let metadata = TableMetadata {
        table: TableInfo {
            name: table_name.to_string(),
            project_id: project.to_string(),
            dataset_id: dataset.to_string(),
            description,
        },
        columns: Vec::new(),
    };
    
    save_table_metadata(&metadata)?;
    println!("Created table metadata: {}.{}.{}", project, dataset, table_name);
    Ok(())
}

fn edit_column_description(project: &str, dataset: &str, table_name: &str, column_name: &str, new_description: Option<String>) -> Result<()> {
    let mut metadata = load_table_metadata(project, dataset, table_name)?;
    
    let column = metadata.columns.iter_mut()
        .find(|c| c.name == column_name)
        .ok_or_else(|| anyhow::anyhow!("Column '{}' not found in table", column_name))?;
    
    let description = if let Some(desc) = new_description {
        desc
    } else {
        print!("Enter description for column '{}': ", column_name);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    };
    
    column.description = if description.is_empty() { None } else { Some(description) };
    save_table_metadata(&metadata)?;
    
    println!("Updated description for column '{}' in {}.{}.{}", 
             column_name, project, dataset, table_name);
    Ok(())
}

fn export_table_metadata(project: &str, dataset: &str, table_name: &str, file_path: Option<&str>, format: &str) -> Result<()> {
    let metadata = load_table_metadata(project, dataset, table_name)?;
    
    let content = match format {
        "json" => serde_json::to_string_pretty(&metadata)?,
        "yaml" => serde_yaml::to_string(&metadata)?,
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
    };
    
    if let Some(path) = file_path {
        std::fs::write(path, content)?;
        println!("Exported to: {}", path);
    } else {
        println!("{}", content);
    }
    
    Ok(())
}

fn import_table_metadata(file_path: &str, force: bool) -> Result<()> {
    let content = std::fs::read_to_string(file_path)?;
    
    let metadata: TableMetadata = if file_path.ends_with(".json") {
        serde_json::from_str(&content)?
    } else {
        serde_yaml::from_str(&content)?
    };
    
    let table_path = get_table_path(&metadata.table.project_id, &metadata.table.dataset_id, &metadata.table.name)?;
    
    if table_path.exists() && !force {
        return Err(anyhow::anyhow!(
            "Table metadata already exists: {}.{}.{} (use --force to overwrite)",
            metadata.table.project_id, metadata.table.dataset_id, metadata.table.name
        ));
    }
    
    save_table_metadata(&metadata)?;
    println!("Imported table metadata: {}.{}.{}", 
             metadata.table.project_id, metadata.table.dataset_id, metadata.table.name);
    
    Ok(())
}

fn set_config_value(key: &str, value: &str) -> Result<()> {
    let mut config = load_config()?;
    
    match key {
        "default_project" => config.default_project = Some(value.to_string()),
        "default_dataset" => config.default_dataset = Some(value.to_string()),
        "output_format" => {
            config.output_format = match value {
                "table" => OutputFormat::Table,
                "json" => OutputFormat::Json,
                "yaml" => OutputFormat::Yaml,
                _ => return Err(anyhow::anyhow!("Invalid output format: {}", value)),
            };
        }
        _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
    }
    
    save_config(&config)?;
    println!("Set {} = {}", key, value);
    Ok(())
}

fn get_config_value(key: &str) -> Result<()> {
    let config = load_config()?;
    
    let value = match key {
        "default_project" => config.default_project.unwrap_or_else(|| "None".to_string()),
        "default_dataset" => config.default_dataset.unwrap_or_else(|| "None".to_string()),
        "output_format" => match config.output_format {
            OutputFormat::Table => "table".to_string(),
            OutputFormat::Json => "json".to_string(),
            OutputFormat::Yaml => "yaml".to_string(),
        },
        _ => return Err(anyhow::anyhow!("Unknown config key: {}", key)),
    };
    
    println!("{}", value);
    Ok(())
}

fn list_config() -> Result<()> {
    let config = load_config()?;
    
    println!("{}", "Configuration".bold());
    println!("{}", "─".repeat(20));
    println!("default_project: {}", config.default_project.unwrap_or_else(|| "None".to_string()));
    println!("default_dataset: {}", config.default_dataset.unwrap_or_else(|| "None".to_string()));
    println!("output_format: {}", match config.output_format {
        OutputFormat::Table => "table",
        OutputFormat::Json => "json", 
        OutputFormat::Yaml => "yaml",
    });
    
    Ok(())
}
