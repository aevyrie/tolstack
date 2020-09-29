use chrono::prelude::*;
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
        let path = path();
        let mut wtr = Writer::from_path(path.clone()).map_err(|_| SaveError::DirectoryError)?;
        for entry in data {
            wtr.serialize(entry)
                .map_err(|_| SaveError::SerializeError)?;
        }
        wtr.flush().map_err(|_| SaveError::WriteError)?;
        open::that(path).map_err(|_| SaveError::OpenError)?;
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
    let now = chrono::offset::Local::now();
    path.push(format!(
        "tolstack_export_{}_{}_{}_{}_{}_{}.csv",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    ));

    path
}
