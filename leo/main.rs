// Copyright (C) 2019-2020 Aleo Systems Inc.
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

pub mod api;
pub mod cmd;
pub mod context;

use anyhow::Error;
use std::process::exit;
use structopt::StructOpt;

use cmd::add::Add;
use cmd::clean::Clean;
use cmd::init::Init;
use cmd::lint::Lint;
use cmd::new::New;
use cmd::Cmd;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(about = "Init Leo project command in current directory")]
    Init {
        #[structopt(flatten)]
        cmd: Init,
    },

    #[structopt(about = "Create Leo project in new directory")]
    New {
        #[structopt(flatten)]
        cmd: New,
    },

    #[structopt(about = "Import package from Aleo PM")]
    Add {
        #[structopt(flatten)]
        cmd: Add,
    },

    #[structopt(about = "Clean current package: remove proof and circuits")]
    Clean {
        #[structopt(flatten)]
        cmd: Clean,
    },

    #[structopt(about = "Lint package code (not implemented)")]
    Lint {
        #[structopt(flatten)]
        cmd: Lint,
    },
}

fn main() {
    let matches = Opt::from_args();

    handle_error(match matches {
        Opt::Init { cmd } => cmd.execute(),
        Opt::New { cmd } => cmd.execute(),
        Opt::Add { cmd } => cmd.execute(),
        Opt::Clean { cmd } => cmd.execute(),
        Opt::Lint { cmd } => cmd.execute(),
    });
}

fn handle_error<T>(res: Result<T, Error>) -> T {
    match res {
        Ok(t) => t,
        Err(err) => {
            eprintln!("error: {:?}", err);
            exit(1);
        }
    }
}
