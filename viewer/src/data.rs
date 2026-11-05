use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct Table {
    pub name: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

pub fn csv_files(folder: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = fs::read_dir(folder)
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "csv"))
        .collect();
    files.sort();
    files
}

pub fn last_change(folder: &Path) -> Option<SystemTime> {
    csv_files(folder).iter().filter_map(|path| fs::metadata(path).ok()?.modified().ok()).max()
}

pub fn load_tables(folder: &Path) -> Vec<Table> {
    csv_files(folder).iter().filter_map(|path| load_table(path)).collect()
}

fn load_table(path: &Path) -> Option<Table> {
    let mut reader = csv::Reader::from_path(path).ok()?;
    let headers = reader.headers().ok()?.iter().map(String::from).collect();
    let rows = reader.records().flatten().map(|record| record.iter().map(String::from).collect()).collect();
    let name = path.file_stem()?.to_string_lossy().to_string();
    Some(Table { name, headers, rows })
}