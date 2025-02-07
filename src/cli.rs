// re-export Parser
pub use clap::Parser;

#[derive(Parser)]
#[command(name = "MozRentPere'o'o")]
#[command(version, about = "Rent a car in the island of Mo'orea", author = "By Rao Nagos")]
pub struct AppCli {
    /// Specify the mode to run the server.
    /// The private mode allows you to manage your clients, reservations and vehicules
    #[arg(short = 'm', long, value_enum, env = "MRP_MODE", default_value_t = Modes::Public)]
    pub mode: Modes,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum Modes {
    /// Run the server as the admin to manage vehicules and reservations
    Private,
    /// Run the server as the main website
    Public,
}
