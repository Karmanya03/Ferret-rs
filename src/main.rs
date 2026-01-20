use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

// Import our custom modules
mod config;
mod duplicates;
mod grep;
mod organize;
mod pentest;
mod search;
mod utils;

use organize::OrganizeCommand;
use search::SearchCommand;

// Main CLI structure
#[derive(Parser)]
#[command(name = "fr")]
#[command(author = "Ferret")]
#[command(version = "0.2.0")]
#[command(about = "Ferret - Fast file finder, organizer, and pentesting tool for Linux/Unix systems", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Available commands
#[derive(Subcommand)]
enum Commands {
    /// Find files with advanced filters and pattern matching
    Find {
        /// Pattern to search for (supports glob patterns)
        pattern: String,

        /// Directory to search in (default: current directory)
        #[arg(short, long, default_value = ".")]
        path: String,

        /// Case-insensitive search (can combine: -irH)
        #[arg(short, long)]
        ignore_case: bool,

        /// Use regex pattern matching (can combine: -irH)
        #[arg(short, long)]
        regex: bool,

        /// File type filter (file, dir, symlink)
        #[arg(short = 't', long)]
        file_type: Option<String>,

        /// Minimum file size (e.g., 1M, 500K, 2G)
        #[arg(long)]
        min_size: Option<String>,

        /// Maximum file size (e.g., 1M, 500K, 2G)
        #[arg(long)]
        max_size: Option<String>,

        /// Modified within last N days
        #[arg(short = 'm', long)]
        modified_days: Option<u64>,

        /// Search recursively (default: true)
        #[arg(short = 'R', long, default_value = "true")]
        recursive: bool,

        /// Maximum depth for recursive search
        #[arg(short = 'd', long)]
        max_depth: Option<usize>,

        /// Show hidden files (can combine: -iH or -irH)
        #[arg(short = 'H', long)]
        hidden: bool,

        /// Output format (default, json, detailed)
        #[arg(short = 'o', long, default_value = "default")]
        output: String,

        /// Execute command on found files
        #[arg(short = 'x', long)]
        exec: Option<String>,

        /// Verbose output (can combine: -vH or -viH)
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Follow symbolic links (can combine: -iHl)
        #[arg(short = 'l', long)]
        follow_links: bool,
    },

    /// Organize files by type, date, or custom rules
    Organize {
        /// Directory to organize
        #[arg(default_value = ".")]
        path: String,

        /// Organization method (type, date, size, extension)
        #[arg(short, long, default_value = "type")]
        method: String,

        /// Output directory for organized files
        #[arg(short, long)]
        output: Option<String>,

        /// Dry run - show what would be done without moving files (can combine: -nrv)
        #[arg(short = 'n', long)]
        dry_run: bool,

        /// Copy files instead of moving (can combine: -crv)
        #[arg(short, long)]
        copy: bool,

        /// Organize recursively (can combine: -rn or -rc)
        #[arg(short, long)]
        recursive: bool,

        /// Include hidden files (can combine: -rH or -nrH)
        #[arg(short = 'H', long)]
        hidden: bool,

        /// Verbose output (can combine: -rvH)
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// Get statistics about files in a directory
    Stats {
        /// Directory to analyze
        #[arg(default_value = ".")]
        path: String,

        /// Analyze recursively (can combine: -rH or -rv)
        #[arg(short, long)]
        recursive: bool,

        /// Include hidden files (can combine: -rH)
        #[arg(short = 'H', long)]
        hidden: bool,

        /// Verbose output (can combine: -rvH)
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// 🔥 Find SUID binaries (setuid - run as owner)
    Suid {
        /// Directory to search (default: /)
        #[arg(default_value = "/")]
        path: String,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Verbose output with permissions
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Output results to file
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// 🔥 Find SGID binaries (setgid - run as group)
    Sgid {
        /// Directory to search (default: /)
        #[arg(default_value = "/")]
        path: String,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Verbose output with permissions
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Output results to file
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// 🔥 Find world-writable files and directories
    Writable {
        /// Directory to search (default: /)
        #[arg(default_value = "/")]
        path: String,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Verbose output with permissions
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Only show directories
        #[arg(short = 'd', long)]
        dirs_only: bool,

        /// Only show files
        #[arg(short = 'f', long)]
        files_only: bool,

        /// Output results to file
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// 🔥 Find files with capabilities (Linux capabilities)
    Caps {
        /// Directory to search (default: /)
        #[arg(default_value = "/")]
        path: String,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Verbose output
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Output results to file
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// 🔥 Find interesting config files (credentials, keys, etc.)
    Configs {
        /// Directory to search (default: /)
        #[arg(default_value = "/")]
        path: String,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Verbose output with file sizes
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Output results to file
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// 🔥 Find recently modified files (useful for detecting changes)
    Recent {
        /// Directory to search (default: /)
        #[arg(default_value = "/")]
        path: String,

        /// Time window in minutes (default: 60)
        #[arg(short = 't', long, default_value = "60")]
        minutes: u64,

        /// Quiet mode - only show file paths
        #[arg(short = 'q', long)]
        quiet: bool,

        /// Verbose output with modification time
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Output results to file
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// 🔥 Quick command execution (run commands directly, optionally redirect output)
    Dn {
        /// Command to run (e.g., "find / -name *.conf")
        #[arg(required = true, trailing_var_arg = true, allow_hyphen_values = true)]
        command: Vec<String>,

        /// Hide stdout (redirect to /dev/null)
        #[arg(short = 's', long)]
        hide_stdout: bool,

        /// Hide stderr (redirect to /dev/null)
        #[arg(short = 'e', long)]
        hide_stderr: bool,

        /// Hide all output (both stdout and stderr)
        #[arg(short = 'q', long)]
        quiet: bool,
    },

    /// List files in directory (like ls command)
    Ls {
        /// Directory to list (default: current directory)
        #[arg(default_value = ".")]
        path: String,

        /// Show all files including hidden (like ls -a)
        #[arg(short = 'a', long)]
        all: bool,

        /// Long format with details (like ls -l)
        #[arg(short = 'l', long)]
        long: bool,

        /// List recursively (like ls -R)
        #[arg(short = 'R', long)]
        recursive: bool,

        /// Human-readable file sizes (like ls -h)
        #[arg(short = 'H', long)]
        human: bool,

        /// Explain permissions in detail (e.g., owner:rw-, group:r--, other:r--)
        #[arg(short = 'e', long)]
        explain_perms: bool,
    },

    /// 🔥 Find duplicate files (size and hash based detection)
    Dupes {
        /// Directory to search (default: current directory)
        #[arg(default_value = ".")]
        path: String,

        /// Minimum file size to consider (e.g., 1M, 100K)
        #[arg(long)]
        min_size: Option<String>,

        /// Search recursively
        #[arg(short = 'r', long)]
        recursive: bool,

        /// Verbose output
        #[arg(short = 'v', long)]
        verbose: bool,

        /// Save report to file
        #[arg(short = 'o', long)]
        output: Option<String>,
    },

    /// 🔍 Search file contents (grep-like functionality)
    Grep {
        /// Pattern to search for
        pattern: String,

        /// Directory to search (default: current directory)
        #[arg(default_value = ".")]
        path: String,

        /// Use regex pattern
        #[arg(short = 'r', long)]
        regex: bool,

        /// Case-insensitive search
        #[arg(short = 'i', long)]
        ignore_case: bool,

        /// Search recursively
        #[arg(short = 'R', long, default_value = "true")]
        recursive: bool,

        /// Filter by file pattern (e.g., *.rs, *.txt)
        #[arg(short = 'g', long)]
        glob: Option<String>,

        /// Number of context lines to show
        #[arg(short = 'C', long, default_value = "0")]
        context: usize,

        /// Verbose output
        #[arg(short = 'v', long)]
        verbose: bool,
    },

    /// ⚙️ Config management
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Initialize config file with defaults
    Init,

    /// Show current config
    Show,

    /// Show config file path
    Path,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Route to the appropriate command handler
    match cli.command {
        Commands::Find {
            pattern,
            path,
            ignore_case,
            regex,
            file_type,
            min_size,
            max_size,
            modified_days,
            recursive,
            max_depth,
            hidden,
            output,
            exec,
            verbose,
            quiet,
            follow_links,
        } => {
            let search_cmd = SearchCommand {
                pattern,
                path,
                ignore_case,
                regex,
                file_type,
                min_size,
                max_size,
                modified_days,
                recursive,
                max_depth,
                hidden,
                output,
                exec,
                verbose,
                quiet,
                follow_links,
            };
            search_cmd.execute()?;
        }

        Commands::Organize {
            path,
            method,
            output,
            dry_run,
            copy,
            recursive,
            hidden,
            verbose,
        } => {
            let organize_cmd = OrganizeCommand {
                path,
                method,
                output,
                dry_run,
                copy,
                recursive,
                hidden,
                verbose,
            };
            organize_cmd.execute()?;
        }

        Commands::Stats {
            path,
            recursive,
            hidden,
            verbose,
        } => {
            utils::show_stats(&path, recursive, hidden, verbose)?;
        }

        Commands::Suid {
            path,
            quiet,
            verbose,
            output,
        } => {
            pentest::find_suid_binaries(&path, quiet, verbose, output)?;
        }

        Commands::Sgid {
            path,
            quiet,
            verbose,
            output,
        } => {
            pentest::find_sgid_binaries(&path, quiet, verbose, output)?;
        }

        Commands::Writable {
            path,
            quiet,
            verbose,
            dirs_only,
            files_only,
            output,
        } => {
            pentest::find_writable(&path, quiet, verbose, dirs_only, files_only, output)?;
        }

        Commands::Caps {
            path,
            quiet,
            verbose,
            output,
        } => {
            pentest::find_capabilities(&path, quiet, verbose, output)?;
        }

        Commands::Configs {
            path,
            quiet,
            verbose,
            output,
        } => {
            pentest::find_configs(&path, quiet, verbose, output)?;
        }

        Commands::Recent {
            path,
            minutes,
            quiet,
            verbose,
            output,
        } => {
            pentest::find_recently_modified(&path, minutes, quiet, verbose, output)?;
        }

        Commands::Dn {
            command,
            hide_stdout,
            hide_stderr,
            quiet,
        } => {
            use std::process::Command as ProcessCommand;

            if command.is_empty() {
                eprintln!("Error: No command provided");
                std::process::exit(1);
            }

            let mut cmd = ProcessCommand::new(&command[0]);
            if command.len() > 1 {
                cmd.args(&command[1..]);
            }

            // Redirect based on flags
            if quiet {
                // Hide both stdout and stderr
                cmd.stdout(std::process::Stdio::null())
                    .stderr(std::process::Stdio::null());
            } else {
                if hide_stdout {
                    cmd.stdout(std::process::Stdio::null());
                }
                if hide_stderr {
                    cmd.stderr(std::process::Stdio::null());
                }
                // If neither flag is set, output is inherited (shows normally)
            }

            match cmd.status() {
                Ok(status) => {
                    if !status.success() {
                        std::process::exit(status.code().unwrap_or(1));
                    }
                }
                Err(e) => {
                    eprintln!("Failed to execute command: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Ls {
            path,
            all,
            long,
            recursive,
            human,
            explain_perms,
        } => {
            utils::list_files(&path, all, long, recursive, human, explain_perms)?;
        }

        Commands::Dupes {
            path,
            min_size,
            recursive,
            verbose,
            output,
        } => {
            let min_bytes = min_size.as_ref().and_then(|s| search::parse_size(s).ok());
            duplicates::find_duplicates(&path, min_bytes, recursive, verbose, output)?;
        }

        Commands::Grep {
            pattern,
            path,
            regex,
            ignore_case,
            recursive,
            glob,
            context: _,
            verbose,
        } => {
            grep::grep_search(grep::GrepOptions {
                pattern,
                path,
                is_regex: regex,
                ignore_case,
                recursive,
                file_pattern: glob,
                verbose,
            })?;
        }

        Commands::Config { action } => match action {
            ConfigAction::Init => {
                let path = config::FerretConfig::init()?;
                println!(
                    "✓ Config file created at: {}",
                    path.display().to_string().cyan()
                );
                println!("  Edit this file to customize file type mappings and settings.");
            }
            ConfigAction::Show => {
                let config = config::FerretConfig::load()?;
                let toml = toml::to_string_pretty(&config)?;
                println!("{}", toml);
            }
            ConfigAction::Path => {
                let path = config::FerretConfig::config_path()?;
                println!("{}", path.display());
            }
        },
    }

    Ok(())
}
