use anyhow::{Context, Result};
use colored::*;
use rayon::prelude::*;
use regex::Regex;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct GrepMatch {
    pub file: PathBuf,
    pub line_number: usize,
    pub line_content: String,
    pub match_positions: Vec<(usize, usize)>,
}

pub struct GrepOptions {
    pub pattern: String,
    pub path: String,
    pub is_regex: bool,
    pub ignore_case: bool,
    pub recursive: bool,
    pub file_pattern: Option<String>,
    pub verbose: bool,
}

/// Search for content in files (like grep)
pub fn grep_search(options: GrepOptions) -> Result<()> {
    if options.verbose {
        println!(
            "\n{} Searching for '{}' in: {}\n",
            "🔍".bold(),
            options.pattern.cyan(),
            options.path.yellow()
        );
    }

    // Compile regex pattern
    let regex_pattern = if options.is_regex {
        if options.ignore_case {
            format!("(?i){}", options.pattern)
        } else {
            options.pattern.clone()
        }
    } else {
        let escaped = regex::escape(&options.pattern);
        if options.ignore_case {
            format!("(?i){}", escaped)
        } else {
            escaped
        }
    };

    let re = Regex::new(&regex_pattern)
        .with_context(|| format!("Invalid regex pattern: {}", options.pattern))?;

    // Collect files to search
    let files: Vec<PathBuf> = WalkDir::new(&options.path)
        .follow_links(false)
        .max_depth(if options.recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            if let Some(ref pattern) = options.file_pattern
                && let Some(name) = e.file_name().to_str()
            {
                return name.contains(pattern);
            }
            true
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    if files.is_empty() {
        println!("{}", "No files to search".yellow());
        return Ok(());
    }

    if options.verbose {
        println!("Searching in {} files...\n", files.len().to_string().cyan());
    }

    // Search in parallel for performance
    let results: Vec<Vec<GrepMatch>> = files
        .par_iter()
        .filter_map(|file| search_file(file, &re).ok())
        .collect();

    // Flatten and display results
    let mut total_matches = 0;
    let mut files_with_matches = 0;

    for file_matches in results {
        if !file_matches.is_empty() {
            files_with_matches += 1;
            total_matches += file_matches.len();

            // Display file name
            println!(
                "{} {}",
                "📄".bold(),
                file_matches[0].file.display().to_string().green().bold()
            );

            // Display matches
            for m in file_matches {
                let line_num = format!("{}:", m.line_number).blue().bold();

                // Highlight matches in the line
                let mut highlighted_line = m.line_content.clone();
                for (start, end) in m.match_positions.iter().rev() {
                    let before = &highlighted_line[..*start];
                    let matched = &highlighted_line[*start..*end];
                    let after = &highlighted_line[*end..];
                    highlighted_line = format!("{}{}{}", before, matched.red().bold(), after);
                }

                println!("  {} {}", line_num, highlighted_line);
            }
            println!();
        }
    }

    // Summary
    if total_matches > 0 {
        println!(
            "{} Found {} matches in {} files",
            "✓".green().bold(),
            total_matches.to_string().cyan().bold(),
            files_with_matches.to_string().yellow()
        );
    } else {
        println!("{}", "No matches found".yellow());
    }

    Ok(())
}

/// Search a single file for matches
fn search_file(path: &Path, re: &Regex) -> Result<Vec<GrepMatch>> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut matches = Vec::new();

    for (line_num, line) in reader.lines().enumerate() {
        let line_content = line?;
        let line_number = line_num + 1;

        // Find all matches in this line
        let mut match_positions = Vec::new();
        for m in re.find_iter(&line_content) {
            match_positions.push((m.start(), m.end()));
        }

        if !match_positions.is_empty() {
            matches.push(GrepMatch {
                file: path.to_path_buf(),
                line_number,
                line_content,
                match_positions,
            });
        }
    }

    Ok(matches)
}

/// Quick content search with count only
#[allow(dead_code)]
pub fn grep_count(
    pattern: &str,
    path: &str,
    is_regex: bool,
    ignore_case: bool,
    recursive: bool,
) -> Result<usize> {
    let regex_pattern = if is_regex {
        if ignore_case {
            format!("(?i){}", pattern)
        } else {
            pattern.to_string()
        }
    } else {
        let escaped = regex::escape(pattern);
        if ignore_case {
            format!("(?i){}", escaped)
        } else {
            escaped
        }
    };

    let re = Regex::new(&regex_pattern)?;

    let files: Vec<PathBuf> = WalkDir::new(path)
        .follow_links(false)
        .max_depth(if recursive { usize::MAX } else { 1 })
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_path_buf())
        .collect();

    let count: usize = files
        .par_iter()
        .map(|file| count_matches_in_file(file, &re).unwrap_or(0))
        .sum();

    Ok(count)
}

#[allow(dead_code)]
fn count_matches_in_file(path: &Path, re: &Regex) -> Result<usize> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut count = 0;

    for line in reader.lines() {
        let line_content = line?;
        count += re.find_iter(&line_content).count();
    }

    Ok(count)
}
