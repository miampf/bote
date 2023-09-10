use std::io;
use std::process::Command;
use std::{fs::File, path::Path};

use git2::build::RepoBuilder;
use rhai::{Engine, EvalAltResult, ImmutableString};
use shlex::Shlex;

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
fn clone_git_repo(repo: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    let mut repo_builder = RepoBuilder::new();
    let clone_result = repo_builder.clone(&repo, Path::new("/tmp/bote"));
    match clone_result {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string().into()),
    }
}

/// execute_system_command() executes a system command in the current working directory.
fn execute_system_command(cmd: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    let mut lex = Shlex::new(cmd.as_str());

    if lex.had_error {
        return Err(format!("failed to parse the given command: {}", cmd).into());
    }

    let program = lex.next();
    if program.is_none() {
        return Err("command is empty".into());
    }

    let res = Command::new(program.unwrap()).args(lex).spawn();

    if let Err(e) = res {
        return Err(e.to_string().into());
    }

    Ok(())
}

/// download_file() downloads a file from a given URL to a path relative to the current working directory.
fn download_file(
    url: ImmutableString,
    filepath: ImmutableString,
) -> Result<(), Box<EvalAltResult>> {
    let request = ureq::get(url.as_str()).call();
    if let Err(e) = request {
        return Err(e.to_string().into());
    }
    let request = request.unwrap();

    let file = File::create(filepath.as_str());
    if let Err(e) = file {
        return Err(e.to_string().into());
    }

    let text = request.into_string();
    if let Err(e) = text {
        return Err(e.to_string().into());
    }
    let text = text.unwrap();

    match io::copy(&mut text.as_bytes(), &mut file.unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("failed to write to file: {}", e).into()),
    }
}

/// change_working_directory() changes the working directory to a new path. This path can be
/// relative to the current working directory.
fn change_working_directory(path: ImmutableString) {
    std::env::set_current_dir(path.as_str()).expect("failed to change working directory");
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
