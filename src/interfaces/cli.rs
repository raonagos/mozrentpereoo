use std::path::PathBuf;

// re-export Parser
pub use clap::Parser;

#[derive(Parser)]
#[command(
    name = "MozRentPere'o'o",
    version,
    about = "Rent a car in the island of Mo'orea.",
    author = "By Rao Nagos"
)]
pub struct AppCli {
    /// Specify the mode to run the server
    /// The `server` mode allows you to manage the website
    #[arg(short = 'm', long, value_enum, env = "MRP_MODE", default_value_t = Modes::Server)]
    pub mode: Modes,

    /// Path to the database directory
    #[arg(long, env = "MRP_DATABASE_PATH", default_value = "/var/lib/rupa")]
    pub database_path: PathBuf,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Modes {
    /// Run the server as the admin to manage the website
    Server,
    /// Run the server as the main website
    Client,
}
