# rs_tree

A cross-platform tree command implementation in Rust that provides Linux-like tree output with box-drawing characters.

## Features

- Cross-platform support (Windows, macOS, Linux)
- Linux-like tree output with box-drawing characters
- Maximum depth control
- Directory-only listing
- Pattern-based exclusion
- Statistical output

## Installation

### From GitHub Repository

```bash
cargo install --git https://github.com/kznagamori/rs_tree
```

### From Source

```bash
git clone https://github.com/kznagamori/rs_tree.git
cd rs_tree
cargo build --release
```

## Usage

### Basic Usage

```bash
# Display tree for current directory
rs_tree

# Display tree for specific directory
rs_tree /path/to/directory
```

### Options

```bash
# Limit tree depth
rs_tree -L 3

# Show directories only
rs_tree -d

# Exclude files/directories matching pattern
rs_tree -I "*.tmp"
rs_tree -I "node_modules"

# Combine options
rs_tree -d -L 2 -I ".*" /path/to/directory
```

### Command Line Options

- `-L, --max-depth <LEVEL>`: Descend only level directories deep
- `-d, --directories-only`: List directories only
- `-I, --exclude <PATTERN>`: Exclude files/directories matching pattern (can be used multiple times)

## Examples

### Basic Tree Output

```
.
├── Cargo.toml
├── LICENSE
├── README.md
└── src
    └── main.rs

1 directory, 3 files
```

### Directory Only Output

```bash
rs_tree -d
```

```
.
└── src

1 directory
```

### With Exclusion Patterns

```bash
rs_tree -I "*.toml" -I "*.md"
```

```
.
├── LICENSE
└── src
    └── main.rs

1 directory, 2 files
```

## Development

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

### Running

```bash
cargo run -- [OPTIONS] [DIRECTORY]
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Dependencies

- [clap](https://crates.io/crates/clap) - Command line argument parsing
- [regex](https://crates.io/crates/regex) - Regular expression support
- [walkdir](https://crates.io/crates/walkdir) - Directory traversal