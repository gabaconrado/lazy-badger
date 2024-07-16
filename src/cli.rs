use std::path::PathBuf;

use clap::Parser;

/// Default scripts root directory
pub const DEFAULT_SCRIPTS_ROOT: &str = "scripts";

/// Lazy-badger CLI
///
/// An utility to run scripts from the working directory
#[derive(Parser)]
#[command(version, about)]
pub struct Cli {
    /// The name of the script
    pub name: String,

    /// The list of arguments to be forwarded to the script
    pub args: Vec<String>,

    /// Root directory to look for scripts
    #[clap(long, default_value = DEFAULT_SCRIPTS_ROOT)]
    pub scripts_root: PathBuf,
}
