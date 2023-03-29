//! `start` subcommand - example of how to write a subcommand

use crate::config::AmaunetConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(clap::Subcommand, Command, Debug)]
pub enum ChainCmd {
    GetBlock,
    GetBlockHash,
    GetFinalizedHead,
    GetHeader,
}

impl Runnable for ChainCmd {
    /// Start the application.
    fn run(&self) {}
}

impl config::Override<AmaunetConfig> for ChainCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, config: AmaunetConfig) -> Result<AmaunetConfig, FrameworkError> {
        Ok(config)
    }
}
