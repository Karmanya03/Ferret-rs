use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::FerretConfig;

#[derive(Debug, Clone)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub hash: Option<String>,
}

#[derive(Debug)]
pub struct DuplicateGroup {
    pub size: u64,
    pub files: Vec<PathBuf>,
    pub total_wasted: u64,
}

/// Find duplicate files using size and hash comparison
pub fn find_duplicates(
    path: &str,
    min_size: Option<u64>,
    recursive: bool,
    verbose: bool,
    output: Option<String>,
) -> Result<()> {
    let config = FerretConfig::load().unwrap_or_default();
    let max_size = config.performance.max_hash_size_mb * 1024 * 1024;

    if verbose {
        println!(
            "\n{} Scanning for duplicates in: {}\n",
            "🔍".bold(),
            path.cyan()
        );
    }

    // Step 1: Collect all files with their sizes (fast)
    let files: Vec<FileInfo> = WalkDir::new(path)
        .follow_links(false)
        .max_depth(if recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|entry| {
            let metadata = entry.metadata().ok()?;
            let size = metadata.len();

            // Filter by minimum size if specified
            if let Some(min) = min_size
                && size < min
            {
                return None;
            }

            Some(FileInfo {
                path: entry.path().to_path_buf(),
                size,
                hash: None,
            })
        })
        .collect();

    if verbose {
        println!("Found {} files to analyze", files.len().to_string().cyan());
    }

    // Step 2: Group by size (files with different sizes can't be duplicates)
    let mut size_groups: HashMap<u64, Vec<FileInfo>> = HashMap::new();
    for file in files {
        size_groups.entry(file.size).or_default().push(file);
    }

    // Filter: only keep size groups with 2+ files
    let potential_dupes: Vec<(u64, Vec<FileInfo>)> = size_groups
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect();

    if potential_dupes.is_empty() {
        println!("{}", "✓ No duplicates found!".green().bold());
        return Ok(());
    }

    if verbose {
        println!(
            "Found {} size groups with potential duplicates\n",
            potential_dupes.len().to_string().yellow()
        );
    }

    // Step 3: Hash files in parallel (only for potential duplicates)
    let pb = if verbose {
        Some(create_progress_bar())
    } else {
        None
    };

    let mut duplicate_groups: Vec<DuplicateGroup> = Vec::new();

    for (size, mut files) in potential_dupes {
        // Skip files that are too large
        if size > max_size {
            if verbose {
                println!(
                    "  {} Skipping {} files of size {} (too large)",
                    "⚠".yellow(),
                    files.len(),
                    humansize::format_size(size, humansize::BINARY)
                );
            }
            continue;
        }

        if let Some(ref pb) = pb {
            pb.set_message(format!(
                "Hashing {} files of size {}",
                files.len(),
                humansize::format_size(size, humansize::BINARY)
            ));
            pb.tick();
        }

        // Hash all files in this size group in parallel
        files.par_iter_mut().for_each(|file| {
            if let Ok(hash) = hash_file(&file.path) {
                file.hash = Some(hash);
            }
        });

        // Group by hash
        let mut hash_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();
        for file in files {
            if let Some(hash) = file.hash {
                hash_groups.entry(hash).or_default().push(file.path);
            }
        }

        // Collect actual duplicates
        for (_, paths) in hash_groups {
            if paths.len() > 1 {
                let total_wasted = size * (paths.len() as u64 - 1);
                duplicate_groups.push(DuplicateGroup {
                    size,
                    files: paths,
                    total_wasted,
                });
            }
        }
    }

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    // Display results
    if duplicate_groups.is_empty() {
        println!("{}", "✓ No duplicates found!".green().bold());
        return Ok(());
    }

    // Sort by wasted space
    duplicate_groups.sort_by(|a, b| b.total_wasted.cmp(&a.total_wasted));

    let total_wasted: u64 = duplicate_groups.iter().map(|g| g.total_wasted).sum();
    let total_dupes: usize = duplicate_groups.iter().map(|g| g.files.len()).sum();

    println!(
        "\n{} Found {} duplicate file groups ({} files total)",
        "✓".green().bold(),
        duplicate_groups.len().to_string().cyan().bold(),
        total_dupes.to_string().yellow()
    );
    println!(
        "{} Total wasted space: {}\n",
        "💾".bold(),
        humansize::format_size(total_wasted, humansize::BINARY)
            .red()
            .bold()
    );

    // Display each group
    for (i, group) in duplicate_groups.iter().enumerate() {
        println!(
            "{}. {} ({} × {} files = {} wasted)",
            i + 1,
            "Duplicate group".bold(),
            humansize::format_size(group.size, humansize::BINARY).cyan(),
            group.files.len().to_string().yellow(),
            humansize::format_size(group.total_wasted, humansize::BINARY).red()
        );

        for (j, path) in group.files.iter().enumerate() {
            let marker = if j == 0 { "�" } else { "📄" };
            println!("   {} {}", marker, path.display().to_string().dimmed());
        }
        println!();
    }

    // Save to file if specified
    if let Some(output_file) = output {
        let mut content = String::new();
        content.push_str("Duplicate File Report\n");
        content.push_str("=====================\n\n");
        content.push_str(&format!(
            "Total duplicate groups: {}\n",
            duplicate_groups.len()
        ));
        content.push_str(&format!("Total duplicate files: {}\n", total_dupes));
        content.push_str(&format!(
            "Total wasted space: {}\n\n",
            humansize::format_size(total_wasted, humansize::BINARY)
        ));

        for (i, group) in duplicate_groups.iter().enumerate() {
            content.push_str(&format!(
                "\nGroup {} (size: {}, wasted: {})\n",
                i + 1,
                humansize::format_size(group.size, humansize::BINARY),
                humansize::format_size(group.total_wasted, humansize::BINARY)
            ));
            content.push_str(&format!("{}:\n", "-".repeat(60)));
            for path in &group.files {
                content.push_str(&format!("  {}\n", path.display()));
            }
        }

        fs::write(&output_file, content)
            .with_context(|| format!("Failed to write output file: {}", output_file))?;

        println!("Report saved to: {}", output_file.cyan());
    }

    Ok(())
}

/// Hash a file using SHA256
fn hash_file(path: &Path) -> Result<String> {
    let mut file =
        fs::File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;

    let mut hasher = Sha256::new();
    let mut buffer = vec![0; 8192]; // 8KB buffer for efficient reading

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

fn create_progress_bar() -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    pb
}
