use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn new() -> Result<Self> {
        let proj_dirs = ProjectDirs::from("com", "siaco", "oddatui")
            .context("Impossibile trovare la cartella di sistema per la cache")?;

        let cache_dir = proj_dirs.cache_dir().to_path_buf();
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }

        Ok(Self { dir: cache_dir })
    }

    /// Legge il dato se esiste ed è più giovane del TTL (12 ore)
    pub fn get(&self, key: &str) -> Option<String> {
        let file_path = self.dir.join(format!("{}.json", key));

        if let Ok(metadata) = fs::metadata(&file_path) {
            if let Ok(modified) = metadata.modified() {
                if let Ok(age) = SystemTime::now().duration_since(modified) {
                    if age < Duration::from_secs(12 * 3600) {
                        return fs::read_to_string(&file_path).ok();
                    }
                }
            }
        }
        None
    }

    /// Salva il dato in cache
    pub fn set(&self, key: &str, data: &str) -> Result<()> {
        let file_path = self.dir.join(format!("{}.json", key));
        fs::write(file_path, data)?;
        Ok(())
    }
}
