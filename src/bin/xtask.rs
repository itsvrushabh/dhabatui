use chrono::Utc;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn repo_root() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn run_cmd(cmd: &str, args: &[&str], cwd: Option<&Path>) -> Result<String, String> {
    let mut c = Command::new(cmd);
    c.args(args);
    if let Some(dir) = cwd {
        c.current_dir(dir);
    }
    let out = c
        .output()
        .map_err(|e| format!("failed to run `{cmd}`: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        return Err(format!("`{cmd}` failed: {stderr}"));
    }
    Ok(String::from_utf8_lossy(&out.stdout).trim().to_string())
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn sync_wiki(args: &[String]) -> Result<(), String> {
    let mut wiki_remote = "https://github.com/itsvrushabh/dhabatui.wiki.git".to_string();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--repo" && i + 1 < args.len() {
            wiki_remote = args[i + 1].clone();
            i += 2;
        } else {
            wiki_remote = args[i].clone();
            i += 1;
        }
    }

    let root = repo_root();
    let wiki_dir = root.join("wiki");
    if !wiki_dir.is_dir() {
        return Err(format!(
            "Error: Wiki source directory '{}' not found.",
            wiki_dir.display()
        ));
    }

    let temp_dir = env::temp_dir().join(format!("dhabatui-wiki-{}", std::process::id()));
    if temp_dir.exists() {
        let _ = fs::remove_dir_all(&temp_dir);
    }
    fs::create_dir_all(&temp_dir).map_err(|e| format!("failed to create temp dir: {e}"))?;

    println!("Syncing DhabaTUI Wiki to: {wiki_remote}");

    let clone_res = Command::new("git")
        .args(["clone", &wiki_remote])
        .arg(&temp_dir)
        .output();

    let cloned = clone_res.map(|o| o.status.success()).unwrap_or(false);
    if cloned {
        println!("Successfully cloned wiki repository.");
    } else {
        println!("Wiki repository not initialized yet or authentication required.");
        println!("Initializing local git repository for initial push...");
        run_cmd("git", &["init", "-b", "master"], Some(&temp_dir))?;
        run_cmd(
            "git",
            &["remote", "add", "origin", &wiki_remote],
            Some(&temp_dir),
        )?;
    }

    copy_dir_all(&wiki_dir, &temp_dir).map_err(|e| format!("failed to copy wiki files: {e}"))?;

    run_cmd(
        "git",
        &["config", "user.name", "github-actions[bot]"],
        Some(&temp_dir),
    )?;
    run_cmd(
        "git",
        &[
            "config",
            "user.email",
            "github-actions[bot]@users.noreply.github.com",
        ],
        Some(&temp_dir),
    )?;
    run_cmd("git", &["add", "-A"], Some(&temp_dir))?;

    let diff_clean = Command::new("git")
        .args(["diff", "--staged", "--quiet"])
        .current_dir(&temp_dir)
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if diff_clean {
        println!("Wiki is already up to date. No changes to push.");
    } else {
        run_cmd(
            "git",
            &[
                "commit",
                "-m",
                "docs(wiki): update wiki documentation from main repository",
            ],
            Some(&temp_dir),
        )?;
        if run_cmd("git", &["push", "-u", "origin", "master"], Some(&temp_dir)).is_err() {
            run_cmd("git", &["push", "-u", "origin", "main"], Some(&temp_dir))?;
        }
        println!("Wiki documentation successfully synced!");
    }

    let _ = fs::remove_dir_all(&temp_dir);
    Ok(())
}

fn prepare_release(args: &[String]) -> Result<(), String> {
    let mut release_type = "patch".to_string();
    let mut dry_run = false;
    let mut custom_version = String::new();

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--type" if i + 1 < args.len() => {
                release_type = args[i + 1].clone();
                i += 2;
            }
            "--dry-run" => {
                dry_run = true;
                i += 1;
            }
            "--version" if i + 1 < args.len() => {
                custom_version = args[i + 1].clone();
                i += 2;
            }
            other => return Err(format!("Unknown argument: {other}")),
        }
    }

    let root = repo_root();
    let latest_tag = run_cmd(
        "git",
        &["describe", "--tags", "--abbrev=0"],
        Some(&root),
    )
    .unwrap_or_else(|_| "v0.1.0".to_string());
    println!("Latest detected tag: {latest_tag}");

    let (new_version, release_title) = if !custom_version.is_empty() {
        (
            custom_version.trim_start_matches('v').to_string(),
            "Manual Release",
        )
    } else {
        let semver = latest_tag.trim_start_matches('v');
        let mut parts = semver.split('.');
        let major: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        let minor: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(1);
        let patch: u64 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);

        match release_type.as_str() {
            "patch" => (format!("{major}.{minor}.{}", patch + 1), "Weekly Patch Release"),
            "minor" => (format!("{major}.{}.0", minor + 1), "Monthly Feature Release"),
            "major_lts" => (format!("{}.0.0", major + 1), "Yearly LTS Release"),
            _ => {
                return Err(format!(
                    "Invalid release type: {release_type}. Expected patch, minor, or major_lts."
                ))
            }
        }
    };

    let new_tag = format!("v{new_version}");
    let today = Utc::now().format("%Y-%m-%d").to_string();

    println!("==============================================");
    println!(" Preparing DhabaTUI Release: {new_tag} ({release_title})");
    println!(" Date: {today}");
    println!(" Dry Run: {dry_run}");
    println!("==============================================");

    let range = format!("{latest_tag}..HEAD");
    let commits_raw =
        run_cmd("git", &["log", &range, "--oneline", "--no-merges"], Some(&root))
            .unwrap_or_default();

    if commits_raw.is_empty() && !dry_run && custom_version.is_empty() {
        println!("No new commits since {latest_tag}. Skipping release.");
        return Ok(());
    }

    let mut feats = Vec::new();
    let mut fixes = Vec::new();
    let mut mobile = Vec::new();
    let mut cis = Vec::new();
    let mut docs = Vec::new();
    let mut others = Vec::new();

    for line in commits_raw.lines().filter(|l| !l.trim().is_empty()) {
        let lower = line.to_lowercase();
        let is_mobile = lower.contains("(android)")
            || lower.contains("(ios)")
            || lower.contains("(mobile)");
        let msg = line.split_once(' ').map(|x| x.1).unwrap_or("");
        if is_mobile
            && (msg.starts_with("ci(") || msg.starts_with("feat(") || msg.starts_with("fix("))
        {
            mobile.push(line);
        } else if msg.starts_with("feat") && !is_mobile {
            feats.push(line);
        } else if msg.starts_with("fix") && !is_mobile {
            fixes.push(line);
        } else if (msg.starts_with("ci") || msg.starts_with("chore") || msg.starts_with("refactor"))
            && !lower.contains("mobile")
            && !lower.contains("android")
            && !lower.contains("ios")
        {
            cis.push(line);
        } else if msg.starts_with("docs") {
            docs.push(line);
        } else {
            others.push(line);
        }
    }

    let mut notes = String::new();
    for (header, items) in [
        ("### 🚀 Features", &feats),
        ("### 🐛 Bug Fixes", &fixes),
        ("### 📱 Mobile (Android / iOS)", &mobile),
        ("### ⚙️ CI / Automation", &cis),
        ("### 📝 Documentation", &docs),
        ("### 🔍 Other Changes", &others),
    ] {
        if !items.is_empty() {
            notes.push('\n');
            notes.push_str(header);
            notes.push('\n');
            for item in items {
                notes.push_str(&format!("- {item}\n"));
            }
        }
    }

    if notes.is_empty() {
        notes.push_str("\n- Maintenance release and dependency updates.\n");
    }

    let release_block = format!("## [{new_version}] - {today} - {release_title}\n{notes}\n");

    println!("Generated Release Notes:");
    println!("----------------------------------------");
    println!("{release_block}");
    println!("----------------------------------------");

    if dry_run {
        println!("Dry run completed. No files modified or committed.");
        return Ok(());
    }

    // 1. Update Cargo.toml [package] version
    let cargo_path = root.join("Cargo.toml");
    let cargo_toml =
        fs::read_to_string(&cargo_path).map_err(|e| format!("failed to read Cargo.toml: {e}"))?;
    let mut in_package = false;
    let mut updated_cargo = String::with_capacity(cargo_toml.len());
    let mut replaced = false;
    for line in cargo_toml.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_package = trimmed == "[package]";
        } else if in_package && !replaced && trimmed.starts_with("version") && trimmed.contains('=')
        {
            updated_cargo.push_str(&format!("version = \"{new_version}\"\n"));
            replaced = true;
            continue;
        }
        updated_cargo.push_str(line);
        updated_cargo.push('\n');
    }
    fs::write(&cargo_path, updated_cargo)
        .map_err(|e| format!("failed to write Cargo.toml: {e}"))?;
    println!("Updated Cargo.toml to version {new_version}");

    // 2. Synchronize Cargo.lock
    run_cmd("cargo", &["check", "--quiet"], Some(&root))?;
    println!("Synchronized Cargo.lock");

    // 3. Prepend release notes to CHANGELOG.md
    let changelog_path = root.join("CHANGELOG.md");
    if changelog_path.exists() {
        let content = fs::read_to_string(&changelog_path)
            .map_err(|e| format!("failed to read CHANGELOG.md: {e}"))?;
        let new_content = if let Some(idx) = content.find("## [Unreleased]") {
            let line_end = content[idx..].find('\n').map(|o| idx + o + 1).unwrap_or(content.len());
            format!(
                "{}\n{}\n{}",
                &content[..line_end],
                release_block.trim_end(),
                &content[line_end..]
            )
        } else {
            format!("# Changelog\n\n{release_block}\n{content}")
        };
        fs::write(&changelog_path, new_content)
            .map_err(|e| format!("failed to write CHANGELOG.md: {e}"))?;
    } else {
        let new_content = format!(
            "# Changelog\n\nAll notable changes to DhabaTUI are documented in this file.\nThe format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),\nand this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).\n\n## [Unreleased]\n\n{release_block}"
        );
        fs::write(&changelog_path, new_content)
            .map_err(|e| format!("failed to write CHANGELOG.md: {e}"))?;
    }
    println!("Updated CHANGELOG.md");
    println!("Release {new_tag} successfully prepared!");

    Ok(())
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: xtask <sync-wiki|prepare-release> [args...]");
        return ExitCode::from(1);
    }

    let res = match args[0].as_str() {
        "sync-wiki" => sync_wiki(&args[1..]),
        "prepare-release" => prepare_release(&args[1..]),
        other => Err(format!("Unknown subcommand: {other}")),
    };

    match res {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}
