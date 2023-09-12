mod buildscript;

use std::fs::File;
use std::io::Read;

use anyhow::bail;
use log::{debug, error, info};
use regex::Regex;
use rhai::{Engine, Scope, AST};
use tempfile::tempdir;
use which::which_re;

use crate::{config, error::Error};

/// run() runs the install subcommand which is used to install a package.
pub fn run() -> Result<(), anyhow::Error> {
    // TODO: implement the install subcommand
    let mut engine = Engine::new();
    buildscript::setup_rhai_engine(&mut engine);

    let mut buildfile = File::open("./build.bote.rhai")?;
    let mut buildscript = String::new();
    buildfile.read_to_string(&mut buildscript)?;

    let ast = engine.compile(buildscript)?;
    let mut scope = Scope::new();

    // set the current working directory to a secure temporary directory
    let working_directory = tempdir()?;
    std::env::set_current_dir(working_directory.path())?;
    debug!(
        "Changed working directory to {}",
        working_directory.path().display()
    );

    execute_build_script(&engine, &ast, &mut scope)?;

    // reset working directory
    std::env::set_current_dir(config::get_app_directory()?)?;

    working_directory.close()?;

    Ok(())
}

fn execute_build_script(
    engine: &Engine,
    ast: &AST,
    scope: &mut Scope,
) -> Result<(), anyhow::Error> {
    let version = engine.call_fn::<String>(scope, ast, "version", ())?;
    info!("Package version: {}", version);

    info!("Checking conflicts..."); // TODO: Implement conflict checks
    let conflicts = engine
        .call_fn::<rhai::Array>(scope, ast, "conflicts", ())?
        .to_vec();
    info!("Conflicts: {:?}", conflicts);

    info!("Checking dependencies..."); // TODO: Implement bote dependency checks
    let bote_dependencies = engine
        .call_fn::<rhai::Array>(scope, ast, "bote_dependencies", ())?
        .to_vec();
    info!("Bote dependencies: {:?}", bote_dependencies);
    let installed_program_dependencies = engine
        .call_fn::<rhai::Array>(scope, ast, "installed_program_dependencies", ())?
        .to_vec();

    for dependency in installed_program_dependencies {
        if which_re(Regex::new(dependency.to_string().as_str())?).is_err() {
            error!(
                "Failed to confirm existense of {}. Please check if you have {} installed.",
                dependency, dependency
            );
            bail!(Error::NotFound {
                whats_missing: dependency.to_string(),
            });
        }
    }

    info!("Preparing installation...");
    engine.call_fn(scope, ast, "prepare", ())?;

    info!("Downloading files...");
    engine.call_fn(scope, ast, "download", ())?;

    info!("Building and installing program...");
    engine.call_fn(scope, ast, "install", ())?;

    Ok(())
}
