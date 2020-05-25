use crate::commands::BuildCommand;
use crate::errors::CLIError;
use crate::files::Manifest;
use crate::{cli::*, cli_types::*};

use clap::ArgMatches;
use std::convert::TryFrom;
use std::env::current_dir;

#[derive(Debug)]
pub struct LoadCommand;

impl CLI for LoadCommand {
    type Options = ();
    type Output = ();

    const NAME: NameType = "load";
    const ABOUT: AboutType = "Install a package from the package manager (*)";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let (_program, _checksum_differs) = BuildCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let _package_name = Manifest::try_from(&path)?.get_package_name();

        log::info!("Unimplemented - `leo deploy`");

        Ok(())
    }
}
