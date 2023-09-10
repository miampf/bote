use rhai::{Engine, ImmutableString};

/// setup_rhai_engine() registers all functions a build script can use for the given engine.
pub fn setup_rhai_engine(engine: &mut Engine) {
    engine
        .register_fn("clone_git_repo", clone_git_repo)
        .register_fn("execute_system_command", execute_system_command)
        .register_fn("download_file", download_file)
        .register_fn("change_working_directory", change_working_directory)
        .register_fn("extract_lzma", extract_lzma)
        .register_fn("extract_bzip2", extract_bzip2)
        .register_fn("extract_deflate", extract_deflate)
        .register_fn("extract_tar_archive", extract_tar_archive);
}

/// clone_git_repo() clones a git repository to the working directory.
fn clone_git_repo(repo: ImmutableString) {
    todo!()
}

/// execute_system_command() executes a system command in the current working directory.
fn execute_system_command(cmd: ImmutableString) {
    todo!()
}

/// download_file() downloads a file from a given URL to the current working directory.
fn download_file(url: ImmutableString) {
    todo!()
}

/// change_working_directory() changes the working directory to a new path. This path can be
/// relative to the current working directory.
fn change_working_directory(path: ImmutableString) {
    todo!()
}

/// extract_lzma() extractes lzma compressed files (usually files ending in .7z or .xz).
fn extract_lzma(file: ImmutableString) {
    todo!()
}

/// extract_bzip2() extractes bzip2 archives (usually files ending in .bz2).
fn extract_bzip2(file: ImmutableString) {
    todo!()
}

/// extract_deflate() extractes DEFLATE archives (usually files ending in .zip or .gz).
fn extract_deflate(file: ImmutableString) {
    todo!()
}

/// extract_tar_archive() extracts a tar archive to the current working directory.
fn extract_tar_archive(file: ImmutableString) {
    todo!()
}
