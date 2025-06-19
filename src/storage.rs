use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;

use crate::config::get_data_dir;
use crate::models::TableMetadata;

pub fn get_table_path(project_id: &str, dataset_id: &str, table_name: &str) -> Result<PathBuf> {
    let data_dir = get_data_dir()?;
    let table_path = data_dir
        .join(project_id)
        .join(dataset_id)
        .join(format!("{}.yaml", table_name));
    Ok(table_path)
}

pub fn load_table_metadata(project_id: &str, dataset_id: &str, table_name: &str) -> Result<TableMetadata> {
    let table_path = get_table_path(project_id, dataset_id, table_name)?;
    
    if !table_path.exists() {
        return Err(anyhow::anyhow!(
            "Table metadata not found: {}.{}.{}",
            project_id, dataset_id, table_name
        ));
    }

    let content = fs::read_to_string(&table_path)
        .with_context(|| format!("Failed to read table metadata: {}", table_path.display()))?;
    
    let metadata: TableMetadata = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse table metadata: {}", table_path.display()))?;
    
    Ok(metadata)
}

pub fn save_table_metadata(metadata: &TableMetadata) -> Result<()> {
    let table_path = get_table_path(
        &metadata.table.project_id,
        &metadata.table.dataset_id,
        &metadata.table.name,
    )?;
    
    // Create parent directories if they don't exist
    if let Some(parent) = table_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    let content = serde_yaml::to_string(metadata)
        .context("Failed to serialize table metadata")?;
    
    fs::write(&table_path, content)
        .with_context(|| format!("Failed to write table metadata: {}", table_path.display()))?;
    
    Ok(())
}

pub fn list_tables(project_id: Option<&str>, dataset_id: Option<&str>) -> Result<Vec<(String, String, String)>> {
    let data_dir = get_data_dir()?;
    let mut tables = Vec::new();

    if !data_dir.exists() {
        return Ok(tables);
    }

    let search_dir = match (project_id, dataset_id) {
        (Some(p), Some(d)) => data_dir.join(p).join(d),
        (Some(p), None) => data_dir.join(p),
        (None, None) => data_dir.clone(),
        (None, Some(_)) => return Err(anyhow::anyhow!("Cannot specify dataset without project")),
    };

    if !search_dir.exists() {
        return Ok(tables);
    }

    fn collect_yaml_files(dir: &PathBuf, base_dir: &PathBuf, tables: &mut Vec<(String, String, String)>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                collect_yaml_files(&path, base_dir, tables)?;
            } else if path.extension().map_or(false, |ext| ext == "yaml") {
                if let Some(table_name) = path.file_stem().and_then(|s| s.to_str()) {
                    let relative_path = path.strip_prefix(base_dir)?;
                    let components: Vec<&str> = relative_path.components()
                        .filter_map(|c| c.as_os_str().to_str())
                        .collect();
                    
                    if components.len() >= 3 {
                        let project = components[0].to_string();
                        let dataset = components[1].to_string();
                        let table = table_name.to_string();
                        tables.push((project, dataset, table));
                    }
                }
            }
        }
        Ok(())
    }

    collect_yaml_files(&search_dir, &data_dir, &mut tables)?;
    tables.sort();
    Ok(tables)
}

pub fn delete_table_metadata(project_id: &str, dataset_id: &str, table_name: &str) -> Result<()> {
    let table_path = get_table_path(project_id, dataset_id, table_name)?;
    
    if !table_path.exists() {
        return Err(anyhow::anyhow!(
            "Table metadata not found: {}.{}.{}",
            project_id, dataset_id, table_name
        ));
    }

    fs::remove_file(&table_path)
        .with_context(|| format!("Failed to delete table metadata: {}", table_path.display()))?;
    
    Ok(())
}