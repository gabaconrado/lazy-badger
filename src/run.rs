use std::{
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

/// The executor of commands
pub const EXECUTOR: &str = "bash";

/// All errors triggered by the script management module
#[derive(Debug, thiserror::Error)]
pub enum RunScriptError {
    /// Script is not executable
    #[error("Given script '{0:?}' is not executable")]
    NotExecutable(PathBuf),
    /// Script is not a file
    #[error("Given path '{0:?}' is not a file")]
    NotAFile(PathBuf),
    /// File was not found
    #[error("Given path '{0:?}' was not found")]
    NotFound(PathBuf),
    /// Error reading the file
    #[error("Error reading file '{0:?}': {1}")]
    ReadFailure(PathBuf, std::io::Error),
    /// Error running the script
    #[error("Error running the script '{0:?}': {1}")]
    RunFailure(PathBuf, std::io::Error),
    /// Error parsing script output
    #[error("Error parsing script '{0:?}' output")]
    OutputParse(PathBuf),
}

/// Runs a script from its path
///
/// Forwards the given arguments to the scripts
///
/// The script will take control over stdin, stdout and stderr during execution
///
/// # Errors
///
/// - Path is not a file;
/// - Path is not found;
/// - Script is not executable;
/// - Error running the script;
/// - Error parsing script output;
pub fn run_script(path: &Path, arguments: &[String]) -> Result<ExitStatus, RunScriptError> {
    if !path.exists() {
        return Err(RunScriptError::NotFound(path.to_path_buf()));
    }
    if !path.is_file() {
        return Err(RunScriptError::NotAFile(path.to_path_buf()));
    }
    let md = path
        .metadata()
        .map_err(|err| RunScriptError::ReadFailure(path.to_path_buf(), err))?;
    if !super::read::is_path_executable(md.permissions()) {
        return Err(RunScriptError::NotExecutable(path.to_path_buf()));
    }

    let status = Command::new(EXECUTOR)
        .arg(path)
        .args(arguments)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|err| RunScriptError::RunFailure(path.to_path_buf(), err))?
        .wait()
        .map_err(|err| RunScriptError::RunFailure(path.to_path_buf(), err))?;

    Ok(status)
}
