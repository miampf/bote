/// Return the version string of the package
fn version() {
  "0.0.1"
}

// Return dependencies of a package as an array of arrays which have
// the DHT key of the library of the dependency as the first entry
// and the package name as the second entry.
fn bote_dependencies() {
  []
}

// Return a list of program names as regexes that should be installed 
// and in the PATH.
fn installed_program_dependencies() {
  ["cargo"]
}

// Return conflicting packages the same way dependencies are formatted.
fn conflicts() {
  []
}

// Runs before any other sources are downloaded. This can be used to set up
// a specific directory structure before downloading anything else.
fn prepare() {}

// Download needed files to the current working directory in here.
fn download() {
  clone_git_repo("https://github.com/miampf/bote.git");
}

// Build and install the package. If you want your package in the PATH,
// you must copy it manually here.
fn install() {
  execute_system_command("cargo install");
}

