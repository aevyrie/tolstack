use super::dialogs;
use crate::ui::components::entry_tolerance::ToleranceEntry;
use serde_derive::*;

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

#[cfg(not(target_arch = "wasm32"))]
impl SavedState {
    fn path() -> std::path::PathBuf {
        let mut path =
            if let Some(project_dirs) = directories::ProjectDirs::from("rs", "", "TolStack") {
                project_dirs.data_dir().into()
            } else {
                std::env::current_dir().unwrap_or(std::path::PathBuf::new())
            };

        path.push("tolstack.json");

        path
    }

    pub async fn load() -> Result<SavedState, LoadError> {
        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(Self::path())
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }

    pub async fn save(self) -> Result<(), SaveError> {
        use async_std::prelude::*;
        let json = serde_json::to_string_pretty(&self).map_err(|_| SaveError::FormatError)?;
        let path = Self::path();
        if let Some(dir) = path.parent() {
            async_std::fs::create_dir_all(dir)
                .await
                .map_err(|_| SaveError::DirectoryError)?;
        }
        {
            let mut file = async_std::fs::File::create(path)
                .await
                .map_err(|_| SaveError::FileError)?;
            file.write_all(json.as_bytes())
                .await
                .map_err(|_| SaveError::WriteError)?;
        }

        // This is a simple way to save at most once every couple seconds
        async_std::task::sleep(std::time::Duration::from_secs(2)).await;

        Ok(())
    }

    pub async fn open() -> Result<SavedState, LoadError> {
        let path = match dialogs::open().await {
            Ok(path) => path,
            Err(error) => {
                println!("{:?}", error);
                return Err(LoadError::FileError);
            }
        };

        use async_std::prelude::*;

        let mut contents = String::new();

        let mut file = async_std::fs::File::open(path)
            .await
            .map_err(|_| LoadError::FileError)?;

        file.read_to_string(&mut contents)
            .await
            .map_err(|_| LoadError::FileError)?;

        serde_json::from_str(&contents).map_err(|_| LoadError::FormatError)
    }
}
