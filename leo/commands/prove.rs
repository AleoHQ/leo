// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use super::setup::Setup;
use crate::{commands::Command, context::Context};
use anyhow::Result;
use structopt::StructOpt;

use leo_package::outputs::ProofFile;

use snarkvm_algorithms::snark::groth16::{Groth16, PreparedVerifyingKey, Proof};
use snarkvm_curves::bls12_377::{Bls12_377, Fr};
use snarkvm_models::algorithms::SNARK;
use snarkvm_utilities::bytes::ToBytes;

use rand::thread_rng;
use tracing::span::Span;

/// Init Leo project command in current directory
#[derive(StructOpt, Debug, Default)]
#[structopt(setting = structopt::clap::AppSettings::ColoredHelp)]
pub struct Prove {
    #[structopt(long = "skip-key-check", help = "Skip key verification on Setup stage")]
    skip_key_check: bool,
}

impl Prove {
    pub fn new(skip_key_check: bool) -> Prove {
        Prove { skip_key_check }
    }
}

impl Command for Prove {
    type Input = <Setup as Command>::Output;
    type Output = (Proof<Bls12_377>, PreparedVerifyingKey<Bls12_377>);

    fn log_span(&self) -> Span {
        tracing::span!(tracing::Level::INFO, "Proving")
    }

    fn prelude(&self) -> Result<Self::Input> {
        Setup::new(self.skip_key_check).execute()
    }

    fn apply(self, ctx: Context, input: Self::Input) -> Result<Self::Output> {
        let (program, parameters, prepared_verifying_key) = input;

        // Get the package name
        let path = ctx.dir()?;
        let package_name = ctx.manifest()?.get_package_name();

        tracing::info!("Starting...");

        let rng = &mut thread_rng();
        let program_proof = Groth16::<Bls12_377, _, Vec<Fr>>::prove(&parameters, &program, rng)?;

        // Write the proof file to the output directory
        let mut proof = vec![];
        program_proof.write(&mut proof)?;
        ProofFile::new(&package_name).write_to(&path, &proof)?;

        Ok((program_proof, prepared_verifying_key))
    }
}
