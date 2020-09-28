use csv::Writer;

#[derive(Debug, Clone)]
pub enum SaveError {
    DirectoryError,
    SerializeError,
    WriteError,
    OpenError,
}

pub async fn serialize_csv(data: Vec<f64>) -> Result<(), SaveError> {
    async {
        let mut wtr = Writer::from_path(path()).map_err(|_| SaveError::DirectoryError)?;
        for entry in data {
            wtr.serialize(entry)
                .map_err(|_| SaveError::SerializeError)?;
        }
        wtr.flush().map_err(|_| SaveError::WriteError)?;
        open::that(path()).map_err(|_| SaveError::OpenError)?;
        Ok(())
    }
    .await
}

fn path() -> std::path::PathBuf {
    let mut path = if let Some(project_dirs) = directories::ProjectDirs::from("rs", "", "TolStack")
    {
        project_dirs.data_dir().into()
    } else {
        std::env::current_dir().unwrap_or(std::path::PathBuf::new())
    };

    path.push("tolstack_export.csv");

    path
}
