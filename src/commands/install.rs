mod buildscript;

use rhai::Engine;

/// run() runs the install subcommand which is used to install a package.
pub fn run() {
    // TODO: implement the install subcommand
    let mut engine = Engine::new();
    buildscript::setup_rhai_engine(&mut engine);
}
