# Ferret v0.2.0 Release Notes

**Release Date:** January 19, 2026

## 🚀 Major Features

### 1. Config File Support 🎨
Take control of your organization! Ferret now supports TOML-based configuration files.

- **Location:** `~/.config/ferret/config.toml` (Linux/macOS) or `%APPDATA%\ferret\config.toml` (Windows)
- **Features:**
  - Custom file type mappings
  - Define your own organization categories
  - Performance tuning options
  - Configurable hash limits for duplicate detection

**Commands:**
```bash
fr config init      # Create config file with defaults
fr config show      # Display current configuration
fr config path      # Show config file location
```

Edit the config file to add your custom file types:
```toml
[file_types]
my-custom-docs = ["custom", "special", "unique"]
```

### 2. Duplicate File Detection 🔍
Find and eliminate duplicate files with blazing speed!

- **Size-based pre-filtering** - Lightning-fast initial scan
- **SHA256 hash comparison** - Cryptographically accurate
- **Parallel processing** - Uses all your CPU cores
- **Smart reporting** - Shows wasted space and generates detailed reports
- **Configurable limits** - Skip files larger than X MB

**Commands:**
```bash
fr dupes                    # Find duplicates in current directory
fr dupes / -r               # Scan entire system
fr dupes --min-size 1M -v   # Only files over 1MB, verbose
fr dupes -o report.txt      # Save report to file
```

**Performance:**
- Processes thousands of files per second
- Multi-threaded hashing
- Configurable max file size to skip giant files

### 3. Content Search (Grep) 🔎
Search inside files like a boss!

- **Regex support** - Full regex pattern matching
- **Case-insensitive** - Optional case sensitivity
- **Parallel scanning** - Search multiple files simultaneously
- **Syntax highlighting** - Matches highlighted in color
- **File filtering** - Only search specific file patterns

**Commands:**
```bash
fr grep "pattern" .                  # Search in current directory
fr grep "TODO" src/ -r               # Recursive search
fr grep "function.*\(" -r --regex    # Regex search
fr grep "password" / -i -R           # Case-insensitive everywhere
fr grep "error" logs/ -g "*.log"     # Only in .log files
```

**Performance:**
- Multi-threaded file reading
- Optimized regex compilation
- Streams large files without loading into memory

### 4. Interactive TUI Mode 🎨
Browse files like it's 1995 (but better)!

- **File browser** - Navigate directories interactively
- **Vim-like keybindings** - j/k for navigation
- **Show/hide hidden files** - Toggle with 'h'
- **Real-time display** - File sizes and types shown live

**Commands:**
```bash
fr tui              # Launch TUI in current directory
fr tui /var/log     # Start in specific directory
```

**Keybindings:**
- `↑/↓` or `j/k` - Navigate up/down
- `Enter` or `l` - Open directory
- `h` - Toggle hidden files
- `r` - Refresh
- `q` or `Esc` - Quit

### 5. Enhanced Organization 📁
Organization now respects your config file!

- Uses custom category mappings from config
- Falls back to sensible defaults
- Fully backward compatible
- No breaking changes

## 🔧 Technical Improvements

### Performance Optimizations
- **Parallel file hashing** with Rayon - up to 8x faster on multi-core systems
- **Parallel grep search** - searches multiple files at once
- **Size-based filtering** - skips expensive operations on small files
- **Buffered I/O** - 8KB buffers for optimal file reading
- **Zero-copy operations** where possible

### Architecture
- New modular structure with separate modules:
  - `config.rs` - Configuration management
  - `duplicates.rs` - Duplicate file detection
  - `grep.rs` - Content search
  - `tui.rs` - Terminal UI

## 📦 Dependencies Added

- `toml 0.8` - Config file parsing
- `sha2 0.10` - Fast SHA256 hashing
- `ratatui 0.28` - Modern terminal UI
- `crossterm 0.28` - Cross-platform terminal
- `dirs 5.0` - Config directory detection

## 🔒 Security & Privacy

- No telemetry or data collection
- All operations are local
- Config files stored in standard locations
- Sensitive files excluded from git (see .gitignore)

## ⚡ Breaking Changes

**None!** This release is 100% backward compatible. All existing commands work exactly as before.

## 🐛 Bug Fixes

- Fixed `dn` command showing output correctly by default
- Improved error handling in file operations
- Better permission handling on Unix systems

## 📝 What's Next (v0.3.0)

Planning for future releases:
- Trash/recycle bin support
- Batch file operations in TUI
- More search operators
- More Advanced Regex options
- Plugin system
- Cloud storage integration

## 🙏 Contributors

Thank you to everyone who uses Ferret and provides feedback!

## 📚 Documentation

Full documentation available at: https://github.com/Karmanya03/Ferret-rs

---

**Upgrade Instructions:**

```bash
# Via Cargo
cargo install ferret-rs --force

# Via AUR
yay -Syu ferret-rs

# Via Homebrew
brew upgrade ferret-rs
```

Enjoy the new features! 🎉
