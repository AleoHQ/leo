use crate::{
    cli::*,
    cli_types::*,
    commands::BuildCommand,
    errors::{CLIError, VerificationKeyFileError},
    files::{Manifest, ProvingKeyFile, VerificationKeyFile},
};
use leo_compiler::{compiler::Compiler, group::edwards_bls12::EdwardsGroupType};

use snarkos_algorithms::snark::{generate_random_parameters, prepare_verifying_key, Parameters, PreparedVerifyingKey};
use snarkos_curves::{bls12_377::Bls12_377, edwards_bls12::Fq};
use snarkos_utilities::bytes::ToBytes;

use clap::ArgMatches;
use rand::thread_rng;
use std::{convert::TryFrom, env::current_dir, time::Instant};

#[derive(Debug)]
pub struct SetupCommand;

impl CLI for SetupCommand {
    type Options = ();
    type Output = (
        Compiler<Fq, EdwardsGroupType>,
        Parameters<Bls12_377>,
        PreparedVerifyingKey<Bls12_377>,
    );

    const ABOUT: AboutType = "Run a program setup";
    const ARGUMENTS: &'static [ArgumentType] = &[];
    const FLAGS: &'static [FlagType] = &[];
    const NAME: NameType = "setup";
    const OPTIONS: &'static [OptionType] = &[];
    const SUBCOMMANDS: &'static [SubCommandType] = &[];

    #[cfg_attr(tarpaulin, skip)]
    fn parse(_arguments: &ArgMatches) -> Result<Self::Options, CLIError> {
        Ok(())
    }

    #[cfg_attr(tarpaulin, skip)]
    fn output(options: Self::Options) -> Result<Self::Output, CLIError> {
        let (program, checksum_differs) = BuildCommand::output(options)?;

        // Get the package name
        let path = current_dir()?;
        let package_name = Manifest::try_from(&path)?.get_package_name();

        // Check if a proving key and verification key already exists
        let keys_exist = ProvingKeyFile::new(&package_name).exists_at(&path)
            && VerificationKeyFile::new(&package_name).exists_at(&path);

        // If keys do not exist or the checksum differs, run the program setup
        if !keys_exist || checksum_differs {
            // Start the timer
            let start = Instant::now();

            // Run the program setup operation
            let rng = &mut thread_rng();
            let parameters = generate_random_parameters::<Bls12_377, _, _>(program.clone(), rng).unwrap();
            let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&parameters.vk);

            // End the timer
            log::info!("Setup completed in {:?} milliseconds", start.elapsed().as_millis());

            // TODO (howardwu): Convert parameters to a 'proving key' struct for serialization.
            // Write the proving key file to the outputs directory
            let mut proving_key = vec![];
            parameters.write(&mut proving_key)?;
            ProvingKeyFile::new(&package_name).write_to(&path, &proving_key)?;

            // Write the proving key file to the outputs directory
            let mut verification_key = vec![];
            prepared_verifying_key.write(&mut verification_key)?;
            VerificationKeyFile::new(&package_name).write_to(&path, &verification_key)?;
        }

        // Read the proving key file from the outputs directory
        let proving_key = ProvingKeyFile::new(&package_name).read_from(&path)?;
        let parameters = Parameters::<Bls12_377>::read(proving_key.as_slice(), true)?;

        // Read the proving key file from the outputs directory
        let prepared_verifying_key = prepare_verifying_key::<Bls12_377>(&parameters.vk);
        {
            // Load the stored verification key from the outputs directory
            let stored_vk = VerificationKeyFile::new(&package_name).read_from(&path)?;

            // Convert the prepared_verifying_key to a buffer
            let mut verification_key = vec![];
            prepared_verifying_key.write(&mut verification_key)?;

            // Check that the constructed prepared verification key matches the stored verification key
            let compare: Vec<(u8, u8)> = verification_key.into_iter().zip(stored_vk.into_iter()).collect();
            for (a, b) in compare {
                if a != b {
                    return Err(VerificationKeyFileError::IncorrectVerificationKey.into());
                }
            }
        }

        log::info!("Completed program setup");

        Ok((program, parameters, prepared_verifying_key))
    }
}
