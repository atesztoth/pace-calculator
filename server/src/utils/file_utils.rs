use std::{env, process};

/// This function resolves a path to a file minding if in a development environment.
/// In development builds, it'll resolve the path relative to CARGO_MANIFEST_DIR,
/// but in release builds it'll resolve it relative to the executable.
pub(crate) fn resolve_file_path(target: &str) -> String {
    if cfg!(debug_assertions) {
        return format!("{}/{}", env!("CARGO_MANIFEST_DIR"), target);
    }

    let exe_dir = env::current_exe().expect("Failed to get current executable path.");
    let exe_dir = exe_dir
        .parent()
        .expect("Failed to get parent directory of current executable.");

    match exe_dir.join(target).to_str() {
        None => {
            eprintln!("Failed to convert path to a string.");
            process::exit(1);
        }
        Some(p) => p.to_string(),
    }
}
