use leo::{cli::*, commands::*, errors::CLIError, logger};

use clap::{App, AppSettings};

#[cfg_attr(tarpaulin, skip)]
fn main() -> Result<(), CLIError> {
    logger::init_logger("leo", 1);

    let arguments = App::new("leo")
        .version("v0.1.0")
        .about("Leo compiler and package manager")
        .author("The Aleo Team <hello@aleo.org>")
        .settings(&[
            AppSettings::ColoredHelp,
            AppSettings::DisableHelpSubcommand,
            AppSettings::DisableVersion,
            AppSettings::SubcommandRequiredElseHelp,
        ])
        .subcommands(vec![
            NewCommand::new().display_order(0),
            InitCommand::new().display_order(1),
            BuildCommand::new().display_order(2),
            LoadCommand::new().display_order(3),
            UnloadCommand::new().display_order(4),
            SetupCommand::new().display_order(5),
            ProveCommand::new().display_order(6),
            RunCommand::new().display_order(7),
            PublishCommand::new().display_order(8),
            DeployCommand::new().display_order(9),
            TestCommand::new().display_order(10),
        ])
        .set_term_width(0)
        .get_matches();

    match arguments.subcommand() {
        ("new", Some(arguments)) => NewCommand::output(NewCommand::parse(arguments)?),
        ("init", Some(arguments)) => InitCommand::output(InitCommand::parse(arguments)?),
        ("build", Some(arguments)) => {
            BuildCommand::output(BuildCommand::parse(arguments)?)?;
            Ok(())
        }
        ("load", Some(arguments)) => LoadCommand::output(LoadCommand::parse(arguments)?),
        ("unload", Some(arguments)) => UnloadCommand::output(UnloadCommand::parse(arguments)?),
        ("setup", Some(arguments)) => {
            SetupCommand::output(SetupCommand::parse(arguments)?)?;
            Ok(())
        }
        ("prove", Some(arguments)) => {
            ProveCommand::output(ProveCommand::parse(arguments)?)?;
            Ok(())
        }
        ("run", Some(arguments)) => RunCommand::output(RunCommand::parse(arguments)?),
        ("publish", Some(arguments)) => PublishCommand::output(PublishCommand::parse(arguments)?),
        ("deploy", Some(arguments)) => DeployCommand::output(DeployCommand::parse(arguments)?),
        ("test", Some(arguments)) => {
            TestCommand::output(TestCommand::parse(arguments)?)?;
            Ok(())
        }
        _ => unreachable!(),
    }
}
