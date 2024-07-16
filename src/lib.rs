//! Lazy badger library
//!
//! All required functionalities to support the lazy-badger binary

/// Supported bash extension of script
const BASH_EXTENSION: &str = "sh";

/// Read scripts implementation
mod read;
/// Run scripts implementation
mod run;

// ********** Re-exports ********** //
pub use self::{
    read::{find_script, list_scripts, FindScriptError, ListScriptsError},
    run::{run_script, RunScriptError, ScriptOutput},
};
