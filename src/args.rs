use std::path::PathBuf;

use clap::{
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct PackFactoryArgs {
    #[clap(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new pack template
    New(NewArgs),
    /// Build the current pack template
    Build,
    /// Remove the target directory
    Clean
}

#[derive(Debug, Args)]
pub struct NewArgs {
    pub path: PathBuf
}

