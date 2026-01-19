# Ferret Release Notes

All release notes for Ferret versions.

---

## [0.2.0] - 2026-01-19

### 🚀 Major Features

#### Config File Support 🎨
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

#### Duplicate File Detection 🔍
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

#### Content Search (Grep) 🔎
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

#### Interactive TUI Mode 🎨
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

#### Enhanced Organization 📁
Organization now respects your config file!

- Uses custom category mappings from config
- Falls back to sensible defaults
- Fully backward compatible
- No breaking changes

### 🔧 Technical Improvements

#### Performance Optimizations
- **Parallel file hashing** with Rayon - up to 8x faster on multi-core systems
- **Parallel grep search** - searches multiple files at once
- **Size-based filtering** - skips expensive operations on small files
- **Buffered I/O** - 8KB buffers for optimal file reading
- **Zero-copy operations** where possible

#### Architecture
- New modular structure with separate modules:
  - `config.rs` - Configuration management
  - `duplicates.rs` - Duplicate file detection
  - `grep.rs` - Content search
  - `tui.rs` - Terminal UI

### 📦 Dependencies Updated

- `toml 0.9` - Config file parsing (updated from 0.8)
- `ratatui 0.30` - Modern terminal UI (updated from 0.28)
- `crossterm 0.29` - Cross-platform terminal (updated from 0.28)
- `dirs 6.0` - Config directory detection (updated from 5.0)
- `sha2 0.10` - Fast SHA256 hashing
- `rayon 1.10` - Parallel processing

### 🔒 Security & Privacy

- No telemetry or data collection
- All operations are local
- Config files stored in standard locations

### ⚡ Breaking Changes

**None!** This release is 100% backward compatible. All existing commands work exactly as before.

### 🐛 Bug Fixes

- Fixed `dn` command showing output correctly by default
- Improved error handling in file operations
- Better permission handling on Unix systems

---

## [0.1.1] - 2026-01-18

### 🔥 Added - Pentesting & Security Features

#### New Commands

**`fr ls`** - Enhanced directory listing command
- Color-coded output (directories, executables, symlinks)
- Long format support with permissions and sizes (like `ls -l`)
- Human-readable file sizes (like `ls -h`)
- Recursive listing (like `ls -R`)
- Show all files including hidden (like `ls -a`)
- Optional permission explanations with `-e` flag
- Works on both Unix and Windows
- Flags: `-a` (all), `-l` (long), `-R` (recursive), `-H` (human-readable), `-e` (explain permissions)

**`fr suid`** - Find SUID binaries (setuid - runs with owner privileges)
- Essential for privilege escalation enumeration
- Replaces: `find / -perm -4000 -type f 2>/dev/null`
- Flags: `-q` (quiet), `-v` (verbose), `-o` (output to file)

**`fr sgid`** - Find SGID binaries (setgid - runs with group privileges)
- Group-based privilege escalation vectors
- Replaces: `find / -perm -2000 -type f 2>/dev/null`
- Flags: `-q` (quiet), `-v` (verbose), `-o` (output to file)

**`fr writable`** - Find world-writable files and directories
- Identify files/dirs anyone can modify
- Perfect for finding persistence locations
- Flags: `-q` (quiet), `-v` (verbose), `-d` (dirs only), `-f` (files only), `-o` (output)

**`fr caps`** - Find files with Linux capabilities
- Often overlooked privilege escalation vector
- Checks for `cap_setuid`, `cap_dac_override`, etc.
- Flags: `-q` (quiet), `-v` (verbose), `-o` (output to file)

**`fr configs`** - Hunt for interesting configuration files and credentials
- Searches for: passwords, SSH keys, certificates, .env files, configs
- Finds: `*.conf`, `*.key`, `*.pem`, `id_rsa`, `passwd`, `shadow`, etc.
- Flags: `-q` (quiet), `-v` (verbose with sizes), `-o` (output to file)

**`fr recent`** - Find recently modified files
- Detect system changes after running exploits
- Monitor file modifications in real-time
- Flags: `-t` (time in minutes), `-q` (quiet), `-v` (verbose), `-o` (output)

**`fr dn`** - Dev null helper for cleaner command output
- Simplified output redirection
- `fr dn command` = `command 2>/dev/null`
- `fr dn -e command` = `command 1>/dev/null` (show errors only)
- No more typing `2>/dev/null` repeatedly!

#### New Module
- Added `src/pentest.rs` module with all security enumeration functions
- Optimized for speed and accuracy
- Progress indicators for long scans
- Colored output for better readability

### 📚 Documentation
- Added comprehensive pentesting section to README.md
- Created PENTEST_CHEATSHEET.md for quick reference
- Included real-world workflow examples
- Added comparison table with traditional commands
- Documented all flags and use cases

### 🎯 Target Audience
- Pentesters and ethical hackers
- Red team operators
- Blue team defenders
- Security researchers
- CTF players
- Linux system administrators

### ⚡ Performance
- All security commands use efficient file system walking
- Built on Rust for blazing fast performance
- Progress indicators for long-running scans
- Parallel processing where applicable
- Cross-platform support with conditional compilation

### 🔧 Improvements
- Updated version to 0.1.1
- Enhanced package description to include pentesting features
- Updated keywords in Cargo.toml
- Cross-platform compatibility for Unix and Windows
- Proper handling of file permissions on different platforms

### 📦 Dependencies
- No new dependencies required
- All features use existing Rust std library and project dependencies

---

## [0.1.0] - 2026-01-17

### Added
- Initial release
- `fr find` - Advanced file search with glob and regex support
- `fr organize` - Smart file organization by type, date, or size
- `fr stats` - Directory statistics and analysis
- Combined flags support (netcat-style: `-ivH`, `-qr`, etc.)
- Multiple output formats (default, detailed, JSON)
- Size filters with human-readable formats (K/M/G)
- Time-based filters
- File type filtering
- Execute commands on found files
- Dry-run mode for organization
- Recursive operations
- Hidden file support
- Follow symlinks option
- Progress indicators
- Colored output
- Cross-platform support (Linux/Unix)

### Features
- Glob pattern matching
- Regex pattern matching
- Case-insensitive search
- Size-based filtering
- Date-based filtering
- File type categorization
- Directory statistics with charts
- Batch file operations

---

## 🙏 Contributors

Thank you to everyone who uses Ferret and provides feedback!

## 📚 Documentation

Full documentation available at: https://github.com/Karmanya03/Ferret-rs
