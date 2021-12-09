use crate::error;
use crate::transport;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File {
    // the path to the file locally
    pub path: PathBuf,
    // the save name of the file in the root
    // directory once it has been transported to the client
    pub alias: Option<String>,
}

impl File {
    pub(crate) fn filename(&self) -> Result<String, error::LoadJobsError> {
        if let Some(alias) = &self.alias {
            Ok(alias.to_string())
        } else {
            let out = self
                .path
                .file_name()
                .ok_or(error::MissingFileNameError::from(self.path.clone()))?
                .to_string_lossy()
                .to_string();
            Ok(out)
        }
    }

    pub(crate) fn normalize_paths(&mut self, base_path: PathBuf) {
        self.path = normalize_pathbuf(self.path.clone(), base_path);
    }
}

pub(crate) async fn load_from_file(
    files: &[File],
) -> Result<Vec<transport::File>, error::LoadJobsError> {
    let mut job_files = vec![];

    for file in files.iter() {
        let file_bytes = tokio::fs::read(&file.path).await.map_err(|e| {
            error::LoadJobsError::from(error::ReadBytesError::new(e, file.path.clone()))
        })?;

        let file_name = file.filename()?;

        job_files.push(transport::File {
            file_name,
            file_bytes,
        });
    }

    Ok(job_files)
}

pub(crate) fn normalize_pathbuf(pathbuf: PathBuf, base_path: PathBuf) -> PathBuf {
    if pathbuf.is_relative() {
        base_path.join(pathbuf)
    } else {
        pathbuf
    }
}
