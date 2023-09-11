mod buildscript;

use regex::Regex;
use rhai::Engine;
use which::which_re;

/// run() runs the install subcommand which is used to install a package.
pub fn run() -> Result<(), anyhow::Error> {
    // TODO: implement the install subcommand
    let mut engine = Engine::new();
    buildscript::setup_rhai_engine(&mut engine);

    Ok(())
}
