use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

// Re-export the modules you need
pub mod builder;
pub mod cmd_args;
pub mod code_utils;
pub mod errors;
pub mod manifest;
pub mod repl;
pub mod shared;
pub mod stdin;
pub mod term_colors;

// Re-export commonly used items for convenience
pub use builder::{execute, gen_build_run};
pub use cmd_args::{get_opt, get_proc_flags, validate_options, Cli, ProcFlags};
pub use code_utils::{
    create_next_repl_file, create_temp_source_file, extract_ast, extract_manifest,
    modified_since_compiled, process_expr,
};
pub use errors::BuildRunError;
pub use repl::run_repl;
pub use shared::{debug_timings, Ast, BuildState, ScriptState};
pub use stdin::{edit_stdin, read_stdin};
pub use term_colors::{nu_resolve_style, MessageLevel};

// Re-export specific items if they are defined in the respective modules
// pub use crate::{gen_build_run, BuildState, DYNAMIC_SUBDIR, REPL_SUBDIR, TEMP_SCRIPT_NAME, TMPDIR};
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const RS_SUFFIX: &str = ".rs";
pub const FLOWER_BOX_LEN: usize = 70;
pub const REPL_SUBDIR: &str = "rs_repl";
pub const DYNAMIC_SUBDIR: &str = "rs_dyn";
pub const TEMP_SCRIPT_NAME: &str = "temp.rs";
pub const TOML_NAME: &str = "Cargo.toml";

lazy_static! {
    pub static ref TMPDIR: PathBuf = env::temp_dir();
}