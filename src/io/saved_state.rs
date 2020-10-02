use super::dialogs;
use crate::ui::components::entry_tolerance::ToleranceEntry;
use serde_derive::*;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedState {
    pub name: String,
    pub tolerances: Vec<ToleranceEntry>,
    pub n_iteration: usize,
    pub assy_sigma: f64,
}

#[derive(Debug, Clone)]
pub enum LoadError {
    FileError,
    FormatError,
}

#[derive(Debug, Clone)]
pub enum SaveError {
    DirectoryError,
    FileError,
    WriteError,
    FormatError,
}

impl Default for SavedState {
    fn default() -> Self {
        SavedState {
            name: "New Project".into(),
            tolerances: Vec::new(),
            n_iteration: 100000,
            assy_sigma: 4.0,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    pub async fn new() -> Result<(Option<std::path::PathBuf>, SavedState), LoadError> {
        Ok((
            None,
            SavedState {
                name: "New Project".into(),
                tolerances: Vec::new(),
                n_iteration: 100000,
                assy_sigma: 4.0,
            },
        ))
    }

    pub async fn save(state: SavedState, path: PathBuf) -> Result<Option<PathBuf>, SaveError> {
        use async_std::prelude::*;
        let json = serde_json::to_string_pretty(&state).map_err(|_| SaveError::FormatError)?;
        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }
        {
            let mut file = async_std::fs::File::create(&path)
                .await
                .map_err(|_| SaveError::FileError)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        Ok(Some(path))
    }

    pub async fn open() -> Result<(Option<PathBuf>, SavedState), LoadError> {
        use async_std::prelude::*;
        let path = match dialogs::open().await {
            Ok(path) => path,
            Err(error) => {
                println!("{:?}", error);
                return Err(LoadError::FileError);
            }
        };
        let mut contents = String::new();
        let mut file = async_std::fs::File::open(&path)
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        match serde_json::from_str(&contents).map_err(|_| LoadError::FormatError) {
            Ok(data) => Ok((Some(path), data)),
            Err(e) => Err(e),
        }
    }

    pub async fn save_as(state: SavedState) -> Result<Option<PathBuf>, SaveError> {
        use async_std::prelude::*;
        let path = match dialogs::save_as().await {
            Ok(path) => path,
            Err(error) => {
                println!("{:?}", error);
                return Err(SaveError::FileError);
            }
        };
        let path = path.with_extension("json");
        let json = serde_json::to_string_pretty(&state).map_err(|_| SaveError::FormatError)?;
        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }
        {
            let mut file = async_std::fs::File::create(&path)
                .await
                .map_err(|_| SaveError::FileError)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        Ok(Some(path))
    }
}
