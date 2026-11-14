// ==================== File reading ====================

use anyhow::{Context, Result};
use std::fs;

pub fn read_file() -> Result<()> {
    let contents = fs::read_to_string("src/file.txt").context("Failed to read src/file.txt")?;
    println!("{contents}");
    Ok(())
}
