# LinkLike ToolBox

A comprehensive toolkit for Link! Like! Love Live! Hasu no Sora School Idol Club game data manipulation and extraction.

## Features

### Asset Management
- **Asset Bundle Encryption/Decryption**: Encrypt and decrypt LinkLike format asset bundles
- **Batch Processing**: Process multiple files and folders at once
- **Format Support**: Handle various LinkLike asset formats

### Chart Data Processing
- **Chart Decompression**: Extract JSON data from compressed chart files (.bytes)
- **Chart Compression**: Compress JSON chart data back to .bytes format
- **Chart Analysis**: Analyze rhythm game chart data with detailed statistics

### Master Data Extraction
- **Auto Update**: Automatically download and update game master data
- **Catalog Processing**: Process and filter game asset catalogs
- **Database Conversion**: Convert encrypted master data to readable formats

### Manifest Handling
- **Manifest Download**: Download and process game manifests
- **Crypto Processing**: Handle encrypted manifest data
- **Asset Tracking**: Track and manage game assets

## Installation

### Prerequisites
- Rust 1.70+ (for building from source)
- Docker (for containerized usage)

### Build from Source
```bash
git clone <repository-url>
cd LinkLikeToolBox
cargo build --release
```

### Docker Usage
```bash
docker-compose up linklike-toolbox
```

## Usage

### CLI Commands

#### Asset Bundle Operations
```bash
# Decrypt asset bundle
./linklike decrypt ab <file_path>

# Encrypt asset bundle  
./linklike crypt ab <file_path>
```

#### Chart Data Operations
```bash
# Decompress chart file (.bytes → .json)
./linklike decrypt chart <file_path>

# Compress chart file (.json → .bytes)
./linklike crypt chart <file_path> [compression_level]
```

#### Master Data Operations
```bash
# Auto update master data
./linklike download auto [options]

# Download specific manifest
./linklike download manifest <real_name> <save_directory>

# Download all assets from catalog
./linklike download assets <catalog_path> <download_directory>
```

#### Auto Update Options
```bash
# Force update even if version is same
./linklike download auto --force

# Download and convert only database files (.tsv)
./linklike download auto --db-only

# Download and convert only chart files (.bytes → .json)  
./linklike download auto --chart-only

# Keep raw downloaded files
./linklike download auto --keep-raw

# Combined options
./linklike download auto --chart-only --keep-raw
./linklike download auto --db-only --force
```

### Python Chart Analysis
```bash
python chart_analyzer.py
```

## Project Structure

```
LinkLikeToolBox/
├── crates/
│   ├── LinkLike_Core/     # Core functionality
│   │   ├── src/
│   │   │   ├── crypt/     # Encryption/decryption modules
│   │   │   ├── fetch/     # Data fetching and processing
│   │   │   ├── master/    # Master data handling
│   │   │   └── manifest.rs # Manifest processing
│   │   └── Cargo.toml
│   └── LinkLike_CLI/      # Command-line interface
│       ├── src/
│       └── Cargo.toml
├── example/               # Sample chart files
├── chart_analyzer.py      # Chart analysis tool
├── docker-compose.yml
├── Dockerfile
└── Makefile
```

## Core Components

### LinkLike_Core
- [`AssetBundle`](crates/LinkLike_Core/src/crypt/asset_bundle.rs): Handle LinkLike asset bundle format
- [`Chart`](crates/LinkLike_Core/src/crypt/chart.rs): Process rhythm game chart data
- [`AssetDecoder`](crates/LinkLike_Core/src/crypt/asset_decoder.rs): Decrypt game assets
- [`AutoUpdater`](crates/LinkLike_Core/src/fetch/auto_update.rs): Automatic data updates
- [`Catalog`](crates/LinkLike_Core/src/manifest.rs): Game asset catalog management

### LinkLike_CLI
- Command-line interface for all operations
- Progress indicators and colored output
- Batch processing capabilities

## Configuration

### Update Options
- `--force`: Force update all assets
- `--db-only`: Update only database files
- `--chart-only`: Update only chart files
- `--keep-raw`: Preserve raw downloaded files

### Compression Levels
Chart compression supports levels 0-9:
- 0: No compression (fastest)
- 6: Default compression (balanced)
- 9: Maximum compression (smallest size)

## Development

### Building
```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build for specific platform
make build-linux
make build-windows
```

### Testing
```bash
cargo test
```

### Docker Development
```bash
# Development container
docker-compose -f docker-compose.yml up dev

# Production build
make docker-build
```

## Chart Analysis

The included Python script provides detailed analysis of rhythm game charts:
- Note timing and type analysis
- BPM and beat information
- Statistical summaries
- CSV export functionality

### Chart Analysis Features
- **Note Classification**: Categorize different note types
- **Timing Analysis**: Analyze note timing patterns
- **Flag Decoding**: Decode note flag information
- **Export Options**: Export analysis to CSV format

## File Formats

### Supported Formats
- `.assetbundle`: LinkLike asset bundles
- `.bytes`: Compressed chart data
- `.json`: Decompressed chart data
- `.tsv`: Master database files

### Chart Data Structure
```json
{
  "Bpms": [...],
  "Offset": 0.0,
  "Beats": [...],
  "Notes": [
    {
      "just": 1000.0,
      "Flags": 123456,
      "Uid": 1,
      "holds": [...]
    }
  ]
}
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is intended for educational and research purposes. Please respect the original game's terms of service.

## Disclaimer

This tool is for educational purposes only. Users are responsible for complying with the game's terms of service and applicable laws.