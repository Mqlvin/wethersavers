use std::fs::{File, OpenOptions, rename};
use std::io::{Read, Write, BufRead, BufReader};
use std::path::Path;
use fs2::FileExt; // bring trait into scope (optional with explicit calls)

use serde::Serialize;
use tokio::task;

// debug build
#[cfg(debug_assertions)]
const PATH: &str = "/tmp/totalsearches";
#[cfg(debug_assertions)]
const UNIQUE_PATH: &str = "/tmp/uniqueids";

// release build
#[cfg(not(debug_assertions))]
const PATH: &str = "/data/totalsearches";
#[cfg(not(debug_assertions))]
const UNIQUE_PATH: &str = "/data/uniqueids";

#[derive(Serialize)]
pub struct StatsResponse {
    pub total_searches: u64,
    pub total_unique_venues: u64
}

pub fn increment_blocking() -> std::io::Result<u64> {
    let lock_path = format!("{}.lock", PATH);
    let lock = OpenOptions::new().read(true).write(true).create(true).open(&lock_path)?;
    fs2::FileExt::lock_exclusive(&lock)?;

    let mut current = 0u64;
    if Path::new(PATH).exists() {
        let mut f = File::open(PATH)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        current = s.trim().parse().unwrap_or(0);
    }

    let next = current + 1;

    let tmp_path = format!("{}.tmp", PATH);
    let mut tf = File::create(&tmp_path)?;
    write!(tf, "{}", next)?;
    tf.sync_all()?;
    rename(&tmp_path, PATH)?;

    if let Some(parent) = Path::new(PATH).parent() {
        let dir = File::open(parent)?;
        dir.sync_all()?;
    }

    fs2::FileExt::unlock(&lock)?;
    Ok(next)
}

pub async fn get_count() -> std::io::Result<u64> {
    let path = PATH.to_string();
    let res = task::spawn_blocking(move || -> std::io::Result<u64> {
        if !Path::new(&path).exists() {
            return Ok(0);
        }
        let mut s = String::new();
        let mut f = File::open(&path)?;
        f.read_to_string(&mut s)?;
        Ok(s.trim().parse::<u64>().unwrap_or(0))
    })
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("JoinError: {}", e)))?;

    res
}


/// Adds `id` if not already present. Returns true if added, false if it already existed.
pub fn add_unique_blocking(id: &str) -> std::io::Result<bool> {
    let lock_path = format!("{}.lock", UNIQUE_PATH);
    let lock = OpenOptions::new().read(true).write(true).create(true).open(&lock_path)?;
    fs2::FileExt::lock_exclusive(&lock)?;

    // Read existing ids into a set-like check
    let mut exists = false;
    if Path::new(UNIQUE_PATH).exists() {
        let f = File::open(UNIQUE_PATH)?;
        let reader = BufReader::new(f);
        for line in reader.lines() {
            if let Ok(l) = line {
                if l.trim() == id {
                    exists = true;
                    break;
                }
            }
        }
    }

    if exists {
        fs2::FileExt::unlock(&lock)?;
        return Ok(false);
    }

    // Append id + newline atomically: write to tmp, rename
    // Read current content (if any), append new id, write tmp, fsync, rename
    let mut content = String::new();
    if Path::new(UNIQUE_PATH).exists() {
        let mut f = File::open(UNIQUE_PATH)?;
        f.read_to_string(&mut content)?;
    }
    content.push_str(id);
    content.push('\n');

    let tmp_path = format!("{}.tmp", UNIQUE_PATH);
    let mut tf = File::create(&tmp_path)?;
    tf.write_all(content.as_bytes())?;
    tf.sync_all()?;
    rename(&tmp_path, UNIQUE_PATH)?;
    if let Some(parent) = Path::new(UNIQUE_PATH).parent() {
        let dir = File::open(parent)?;
        dir.sync_all()?;
    }

    fs2::FileExt::unlock(&lock)?;
    Ok(true)
}

/// Returns number of unique IDs (0 if file missing)
pub fn unique_count_blocking() -> std::io::Result<u64> {
    if !Path::new(UNIQUE_PATH).exists() {
        return Ok(0);
    }
    let f = File::open(UNIQUE_PATH)?;
    let reader = BufReader::new(f);
    let mut count = 0u64;
    for line in reader.lines() {
        if let Ok(l) = line {
            if !l.trim().is_empty() {
                count += 1;
            }
        }
    }
    Ok(count)
}
