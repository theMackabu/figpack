mod cli;

use clap::{Parser, Subcommand};
use clap_verbosity_flag::Verbosity;
use exact_panic::setup_panic;
use macros_rs::str;
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
    Add {
        #[command()]
        name: String,
    },
    /// Remove a dependency
    Remove {
        #[command()]
        name: String,
    },
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

    match &cli.command {
        /* package.yml */
        Some(Commands::Init) => cli::create_project_yml(),

        /* registry */
        Some(Commands::Login) => vendor::registry::auth::login(),
        Some(Commands::Logout) => vendor::registry::auth::logout(),
        Some(Commands::Publish) => vendor::registry::package::publish(),

        /* package management */
        Some(Commands::Clean) => vendor::registry::manager::clean(),
        Some(Commands::Install) => vendor::registry::manager::install(),
        Some(Commands::Add { name }) => vendor::registry::manager::add(name, true),
        Some(Commands::Remove { name }) => vendor::registry::manager::remove(name),

        None => {}
    }
}
