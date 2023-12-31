use std::io;
use std::io::{Read, Write};
use std::process::Command;
use std::{fs::File, path::Path};

use bzip2_rs::DecoderReader;
use git2::build::RepoBuilder;
use log::{error, info};
use lzma::LzmaReader;
use rhai::packages::Package;
use rhai::{Engine, EvalAltResult, ImmutableString};
use rhai_url::UrlPackage;
use shlex::Shlex;
use tar::Archive;
use zip::ZipArchive;

// HACK: The current code style for the functions is really ugly. I should refactor it by
// implementing the From trait for the bote error type to Box<EvalAltResult> and by extracting
// common functionality into their own functions. But for now, this works.

/// setup_rhai_engine() registers all functions and external packages a build script can use for the given engine.
pub fn setup_rhai_engine(engine: &mut Engine) {
    let url = UrlPackage::new();

    engine
        .register_fn("clone_git_repo", clone_git_repo)
        .register_fn("execute_system_command", execute_system_command)
        .register_fn("download_file", download_file)
        .register_fn("change_working_directory", change_working_directory)
        .register_fn("extract_lzma", extract_lzma)
        .register_fn("extract_bzip2", extract_bzip2)
        .register_fn("extract_zip", extract_zip)
        .register_fn("extract_tar_archive", extract_tar_archive);

    url.register_into_engine(engine);
}

/// clone_git_repo() clones a git repository to a path relative to the working directory.
fn clone_git_repo(repo: ImmutableString, path: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    info!("Cloning repository {}", repo);

    let mut repo_builder = RepoBuilder::new();

    let current_dir = std::env::current_dir();
    if let Err(e) = current_dir {
        error!("Failed to obtain current working directory");
        return Err(e.to_string().into());
    }
    let current_dir = current_dir.unwrap();

    let clone_result = repo_builder.clone(&repo, &current_dir.join(path.to_string()));
    match clone_result {
        Ok(_) => Ok(()),
        Err(e) => {
            error!(
                "Failed to clone the remote git repository ({}): {}",
                repo, e
            );
            Err(e.to_string().into())
        }
    }
}

/// execute_system_command() executes a system command in the current working directory.
fn execute_system_command(cmd: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    info!("Executing command {}", cmd);

    let mut lex = Shlex::new(cmd.as_str());

    if lex.had_error {
        error!("Error parsing the command {}", cmd);
        return Err(format!("failed to parse the given command: {}", cmd).into());
    }

    let program = lex.next();
    if program.is_none() {
        error!("No command was provided in build script function call");
        return Err("command is empty".into());
    }

    let child_command = Command::new(program.unwrap()).args(lex).spawn();
    if let Err(e) = child_command {
        error!("Failed to execute command {}: {}", cmd, e);
        return Err(e.to_string().into());
    }
    let mut child_command = child_command.unwrap();

    if let Err(e) = child_command.wait() {
        error!("Command execution of \"{}\" failed: {}", cmd, e);
        return Err(e.to_string().into());
    }

    Ok(())
}

/// download_file() downloads a file from a given URL to a path relative to the current working directory.
fn download_file(
    url: ImmutableString,
    filepath: ImmutableString,
) -> Result<(), Box<EvalAltResult>> {
    info!("Downloading file from {} to {}", url, filepath);

    let request = ureq::get(url.as_str()).call();
    if let Err(e) = request {
        error!("Failed to request {}: {}", url, e);
        return Err(e.to_string().into());
    }
    let request = request.unwrap();

    let file = File::create(filepath.as_str());
    if let Err(e) = file {
        error!("Failed to create file {}", filepath);
        return Err(e.to_string().into());
    }

    let text = request.into_string();
    if let Err(e) = text {
        error!("Failed to obtain text from request to {}: {}", url, e);
        return Err(e.to_string().into());
    }
    let text = text.unwrap();

    match io::copy(&mut text.as_bytes(), &mut file.unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to write to file {}: {}", filepath, e);
            Err(format!("failed to write to file: {}", e).into())
        }
    }
}

/// change_working_directory() changes the working directory to a new path. This path can be
/// relative to the current working directory.
fn change_working_directory(path: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    info!("Changing working directory to {}", path);

    let result = std::env::set_current_dir(path.as_str());
    if let Err(e) = result {
        error!("Failed to change working directory to {}: {}", path, e);
        return Err(e.to_string().into());
    }

    Ok(())
}

/// extract_lzma() extractes lzma compressed files (usually files ending in .7z or .xz) to a path
/// relative to the current working directory.
fn extract_lzma(file: ImmutableString, path: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    info!("Extracting LZMA archive {} to {}", file, path);

    let archive = File::open(file.as_str());
    if let Err(e) = archive {
        error!("Failed to open file {}: {}", file, e);
        return Err(e.to_string().into());
    }
    let archive = archive.unwrap();

    let decompressor = LzmaReader::new_decompressor(archive);
    if let Err(e) = decompressor {
        error!("Failed to create decompressor for file {}: {}", file, e);
        return Err(e.to_string().into());
    }
    let mut decompressor = decompressor.unwrap();

    let mut out_buffer = Vec::new();

    let decompression_result = decompressor.read_to_end(&mut out_buffer);
    if let Err(e) = decompression_result {
        error!("Failed to decompress {}: {}", file, e);
        return Err(e.to_string().into());
    }

    let out_file = File::create(path.to_string());
    if let Err(e) = out_file {
        error!(
            "Failed to create output file {} for archive {}: {}",
            path, file, e
        );
        return Err(e.to_string().into());
    }
    let mut out_file = out_file.unwrap();

    let write_result = out_file.write_all(out_buffer.as_slice());
    if let Err(e) = write_result {
        error!(
            "Failed to write to output file {} for archive {}: {}",
            path, file, e
        );
        return Err(e.to_string().into());
    }

    Ok(())
}

/// extract_bzip2() extractes bzip2 archives (usually files ending in .bz2) to a path relative to
/// the current working directory.
fn extract_bzip2(file: ImmutableString, path: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    info!("Extracting bzip2 archive {} to {}", file, path);

    let archive = File::open(file.as_str());
    if let Err(e) = archive {
        error!("Failed to open file {}: {}", file, e);
        return Err(e.to_string().into());
    }
    let archive = archive.unwrap();

    let output_file = File::create(path.as_str());
    if let Err(e) = output_file {
        error!("Failed to create file {}: {}", path, e);
        return Err(e.to_string().into());
    }
    let mut output_file = output_file.unwrap();

    let mut decompressed_reader = DecoderReader::new(archive);

    let result = io::copy(&mut decompressed_reader, &mut output_file);
    if let Err(e) = result {
        error!("Failed to write to {}: {}", path, e);
        return Err(e.to_string().into());
    }

    Ok(())
}

/// extract_zip() extractes zip archives to a path relative to the current working directory.
fn extract_zip(file: ImmutableString, path: ImmutableString) -> Result<(), Box<EvalAltResult>> {
    info!("Extracting zip archive {} to {}", file, path);

    let archive = File::open(file.as_str());
    if let Err(e) = archive {
        error!("Failed to open file {}: {}", file, e);
        return Err(e.to_string().into());
    }
    let archive = archive.unwrap();

    let zip = ZipArchive::new(archive);
    if let Err(e) = zip {
        error!("Failed to extract archive: {}", e);
        return Err(e.to_string().into());
    }
    let mut zip = zip.unwrap();

    for i in 0..zip.len() {
        let file = zip.by_index(i);
        if let Err(e) = file {
            error!("Failed to extract a file from archive: {}", e);
            return Err(e.to_string().into());
        }
        let mut file = file.unwrap();

        info!("Extracting file {}", file.name());

        let output_file = File::create(path.to_string() + file.name());
        if let Err(e) = output_file {
            error!(
                "Failed to create file {}: {}",
                path.to_string() + file.name(),
                e
            );
            return Err(e.to_string().into());
        }
        let mut output_file = output_file.unwrap();

        if let Err(e) = std::io::copy(&mut file, &mut output_file) {
            error!(
                "Failed to write to file {}: {}",
                path.to_string() + file.name(),
                e
            );
            return Err(e.to_string().into());
        }
    }

    Ok(())
}

/// extract_tar_archive() extracts a tar archive to the current working directory.
fn extract_tar_archive(
    file: ImmutableString,
    path: ImmutableString,
) -> Result<(), Box<EvalAltResult>> {
    info!("Extracting tar archive {} to {}", file, path);

    let archive_file = File::open(file.as_str());
    if let Err(e) = archive_file {
        error!("Failed to open file {}: {}", file, e);
        return Err(e.to_string().into());
    }
    let mut archive = Archive::new(archive_file.unwrap());

    let archive_entries = archive.entries();
    if let Err(e) = archive_entries {
        error!("Failed to get entries from archive {}: {}", file, e);
        return Err(e.to_string().into());
    }
    let archive_entries = archive_entries.unwrap();

    for file in archive_entries {
        if let Err(e) = file {
            error!("Failed to retrieve file from tar archive: {}", e);
            return Err(e.to_string().into());
        }
        let mut file = file.unwrap();

        let filepath = file.path();
        if let Err(e) = filepath {
            error!("Failed to retrieve path from file of archive: {}", e);
            return Err(e.to_string().into());
        }
        let filepath = filepath.unwrap();
        let filepath = filepath.to_str();
        if filepath.is_none() {
            error!("Failed to convert filepath to string");
            return Err("failed to convert filepath to string".into());
        }
        let filepath = filepath.unwrap();

        info!(
            "Extracting {:?} to {}",
            filepath,
            path.to_string() + filepath
        );

        if let Err(e) = file.unpack_in(path.to_string()) {
            error!("Failed to unpack archive entry: {}", e);
            return Err(e.to_string().into());
        }
    }

    Ok(())
}
