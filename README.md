# bq-meta

A fast and efficient CLI tool for managing BigQuery table metadata and column descriptions using local YAML files.

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

- Code was generated by claude code
- Binary is not tested yet....


## Features

- 🚀 **Fast**: Built with Rust for high performance
- 📁 **Local Storage**: Manage metadata in local YAML files with hierarchical structure
- 🔍 **Advanced Search**: Search by table names, descriptions, column names, and column descriptions
- 🎨 **Rich Output**: Colored terminal output with multiple format options (table, JSON, YAML)
- ⚙️ **Flexible Configuration**: Customizable data paths via environment variables
- 📝 **Easy Editing**: Interactive column description editing
- 📤 **Import/Export**: Support for importing and exporting metadata in JSON/YAML formats

## Installation

### Prerequisites

- Rust 1.70 or later

### Build from Source

```bash
git clone https://github.com/yourusername/bq-meta.git
cd bq-meta
cargo build --release
```

The binary will be available at `target/release/bq-meta`.

### Install via Cargo

```bash
cargo install bq-meta
```

## Quick Start

1. **Initialize the metadata directory:**
   ```bash
   bq-meta init
   ```

2. **Create your first table metadata:**
   ```bash
   bq-meta create my-project.analytics.user_events --description "User event tracking table"
   ```

3. **List all tables:**
   ```bash
   bq-meta list
   ```

4. **Search for tables:**
   ```bash
   bq-meta search user
   ```

## Configuration

### Environment Variables

- `BQ_META_PATH`: Set custom path for metadata storage (default: `~/.bq-meta`)

```bash
export BQ_META_PATH="/path/to/your/metadata"
bq-meta init
```

### Data Structure

```
${BQ_META_PATH}/
├── config.yaml                    # Global configuration
└── data/
    └── {project_id}/
        └── {dataset_id}/
            ├── table1.yaml         # Table metadata
            ├── table2.yaml
            └── ...
```

### YAML File Format

```yaml
table:
  name: user_events
  project_id: my-project
  dataset_id: analytics
  description: "User event tracking table"

columns:
  - name: user_id
    type: STRING
    description: "Unique user identifier"
  - name: event_name
    type: STRING
    description: "Name of the event"
  - name: timestamp
    type: TIMESTAMP
    description: "Event occurrence time"
```

## Commands

### Basic Commands

```bash
# Initialize metadata directory
bq-meta init [--path /custom/path]

# List tables
bq-meta list [--project PROJECT] [--dataset DATASET] [--output FORMAT]

# Show table details
bq-meta show PROJECT.DATASET.TABLE [--output FORMAT]

# Describe table columns
bq-meta describe PROJECT.DATASET.TABLE [--output FORMAT]

# Create new table metadata
bq-meta create PROJECT.DATASET.TABLE [--description "Table description"]
```

### Search Commands

```bash
# Search table names (default)
bq-meta search PATTERN

# Search all fields (table names, descriptions, columns)
bq-meta search --all PATTERN

# Search specific fields
bq-meta search --desc PATTERN           # Table descriptions only
bq-meta search --column PATTERN         # Column names only
bq-meta search --col-desc PATTERN       # Column descriptions only

# Advanced search options
bq-meta search --regex "^user_.*"       # Regular expression
bq-meta search --case-sensitive USER    # Case sensitive
bq-meta search --project my-project user # Filter by project
```

### Edit Commands

```bash
# Edit column description interactively
bq-meta edit PROJECT.DATASET.TABLE COLUMN_NAME

# Set column description directly
bq-meta edit PROJECT.DATASET.TABLE COLUMN_NAME --description "New description"
```

### Import/Export Commands

```bash
# Export table metadata
bq-meta export PROJECT.DATASET.TABLE [--file output.yaml] [--format yaml|json]

# Import table metadata
bq-meta import metadata.yaml [--force]
```

### Configuration Commands

```bash
# Set configuration
bq-meta config set default_project my-project
bq-meta config set output_format json

# Get configuration
bq-meta config get default_project

# List all configuration
bq-meta config list
```

## Examples

### Managing Table Metadata

```bash
# Create a new table with description
bq-meta create my-project.sales.orders --description "Customer order data"

# Add column metadata by editing the YAML file or using the edit command
bq-meta edit my-project.sales.orders order_id --description "Unique order identifier"
bq-meta edit my-project.sales.orders customer_id --description "Customer identifier"
bq-meta edit my-project.sales.orders total_amount --description "Order total amount in USD"
```

### Searching

```bash
# Find all tables with "user" in the name
bq-meta search user

# Find tables with "analytics" in description
bq-meta search --desc analytics

# Find all columns containing "id"
bq-meta search --column id

# Complex search with regex
bq-meta search --regex --all "user.*event"
```

### Batch Operations

```bash
# Export all tables in a dataset
for table in $(bq-meta list --project my-project --dataset analytics --output json | jq -r '.[].2'); do
  bq-meta export my-project.analytics.$table --file "backups/${table}.yaml"
done

# Import multiple tables
for file in backups/*.yaml; do
  bq-meta import "$file"
done
```

## Output Formats

All commands support multiple output formats:

- `table` (default): Human-readable colored table format
- `json`: JSON format for programmatic use
- `yaml`: YAML format for easy editing

```bash
bq-meta list --output json
bq-meta search user --output yaml
bq-meta describe my-project.analytics.events --output table
```

## Development

### Project Structure

```
bq-meta/
├── Cargo.toml              # Dependencies and metadata
├── src/
│   ├── main.rs             # Entry point and CLI handlers
│   ├── cli.rs              # CLI definitions (clap)
│   ├── config.rs           # Configuration management
│   ├── storage.rs          # YAML file operations
│   ├── search.rs           # Search functionality
│   ├── models.rs           # Data structures
│   └── lib.rs              # Library exports
├── tests/                  # Integration tests
└── README.md
```

### Running Tests

```bash
cargo test
```

### Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature-name`
3. Make your changes and add tests
4. Run tests: `cargo test`
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [clap](https://github.com/clap-rs/clap) for CLI parsing
- Uses [serde](https://serde.rs/) for YAML serialization
- Colored output powered by [colored](https://github.com/mackwic/colored)
- Async runtime provided by [tokio](https://tokio.rs/)