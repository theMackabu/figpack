mod cli;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use exact_panic::setup_panic;
use macros_rs::{str, string};
use std::env;

#[derive(Parser)]
#[command(version = str!(cli::get_version(false)))]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[clap(flatten)]
    verbose: Verbosity,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new package.yml
    Init,
    /// Save an auth token for the registry locally
    Login,
    /// Remove the local auth token for the registry
    Logout,
    /// Package and upload this package to the registry
    Publish,
    /// Install all dependencies defined in package.yml
    Install,
    /// Add a new dependency
    Add,
    /// Remove a dependency
    Remove,
    /// Remove unused dependencies
    Clean,
}

fn main() {
    vendor::set_name("fpm");
    vendor::set_registry("https://r.justjs.dev");

    setup_panic!(Metadata {
        name: "The Figura Package Manager",
        short_name: "figpack",
        version: env!("CARGO_PKG_VERSION"),
        repository: "https://github.com/theMackabu/figpack"
    });

    let cli = Cli::parse();
    env_logger::Builder::new().filter_level(cli.verbose.log_level_filter()).init();
}
