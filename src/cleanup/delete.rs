use serde::{Deserialize, Serialize};
use std::fs;

use super::model::CleanupTask;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct DeleteReport {
    pub path: String,
    pub deleted: bool,
    pub error: Option<String>,
}

pub fn delete_folders(tasks: &[CleanupTask]) -> Vec<DeleteReport> {
    tasks
        .iter()
        .map(|task| match fs::remove_dir_all(&task.path) {
            Ok(_) => DeleteReport {
                path: task.path.display().to_string(),
                deleted: true,
                error: None,
            },
            Err(error) => DeleteReport {
                path: task.path.display().to_string(),
                deleted: false,
                error: Some(error.to_string()),
            },
        })
        .collect()
}

pub fn delete_paths(paths: &[String]) -> Vec<DeleteReport> {
    paths
        .iter()
        .map(|path| match fs::remove_dir_all(path) {
            Ok(_) => DeleteReport {
                path: path.clone(),
                deleted: true,
                error: None,
            },
            Err(error) => DeleteReport {
                path: path.clone(),
                deleted: false,
                error: Some(error.to_string()),
            },
        })
        .collect()
}
