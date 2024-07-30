use std::{
    fs::Permissions,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
};

/// All errors triggered by the find script feature
#[derive(Debug, thiserror::Error)]
pub enum FindScriptError {
    /// Multiple scripts were found
    #[error("Multiple({1}) scripts found with name '{0}'")]
    MultipleFound(String, usize),
    /// List scripts error
    #[error(transparent)]
    ListScriptsError(#[from] ListScriptsError),
}

/// All errors triggered by the list scripts feature
#[derive(Debug, thiserror::Error)]
pub enum ListScriptsError {
    /// Path is not a directory
    #[error("Path '{0:?}' is not a directory")]
    NotADir(PathBuf),
    /// Error reading the path
    #[error("Error reading directory: {0}")]
    ReadDirFailure(std::io::Error),
}

/// Finds a script from its name and the root directory
///
/// Returns the path to the script if found
///
/// # Errors
///
/// - Error reading any sub-directory;
/// - Passed-in path is not a directory;
/// - More than one script was found;
pub fn find_script(root: &Path, name: String) -> Result<Option<PathBuf>, FindScriptError> {
    let all_scripts = list_scripts(root)?;
    let matching_scripts = all_scripts
        .into_iter()
        .filter(|(n, _)| *n == name)
        .collect::<Vec<_>>();
    match matching_scripts.as_slice() {
        [(_, path)] => Ok(Some(path.to_path_buf())),
        [] => Ok(None),
        multiple => Err(FindScriptError::MultipleFound(name, multiple.len())),
    }
}

/// Lists all available scripts in a directory and its sub-directories
///
/// Returns the name of the script and its full path
///
/// Transverse the passed-in directory and find all files that:
///   - Have the [`BASH_EXTENSION`] extension;
///   - Are executable;
///
/// # Notes
///
/// - Ignores files that fail to be read
///
/// # Errors
///
/// - Error reading any sub-directory;
/// - Passed-in path is not a directory;
pub fn list_scripts(root: &Path) -> Result<Vec<(String, PathBuf)>, ListScriptsError> {
    let mut found_scripts = Vec::new();
    let sub_dirs = root
        .read_dir()
        .map_err(ListScriptsError::ReadDirFailure)?
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_dir())
        .collect::<Vec<_>>();
    for sub_dir in sub_dirs {
        let sub_scripts = list_scripts(&sub_dir.path())?;
        found_scripts.extend(sub_scripts);
    }
    found_scripts.extend(list_scripts_at_directory(root)?);

    Ok(found_scripts)
}

/// Lists all scripts in the given directory
///
/// Ignores files that cannot be read
fn list_scripts_at_directory(dir: &Path) -> Result<Vec<(String, PathBuf)>, ListScriptsError> {
    if !dir.is_dir() {
        return Err(ListScriptsError::NotADir(dir.to_path_buf()));
    }
    let scripts = dir
        .read_dir()
        .map_err(ListScriptsError::ReadDirFailure)?
        .filter_map(|f| f.ok())
        .filter(|f| f.path().is_file())
        .filter(|f| {
            let Ok(m) = f.metadata() else {
                return false;
            };
            is_path_executable(m.permissions())
        })
        .filter(|f| {
            let Some(e) = f
                .path()
                .extension()
                .map(|e| e.to_string_lossy().to_string())
            else {
                return false;
            };
            e == super::BASH_EXTENSION
        })
        .filter_map(|f| {
            let name = f.path().file_stem()?.to_string_lossy().to_string();
            Some((name, f.path()))
        })
        .collect::<Vec<_>>();

    Ok(scripts)
}

/// Returns if the given path is executable
pub(crate) fn is_path_executable(permissions: Permissions) -> bool {
    permissions.mode() & 0o111 != 0
}
