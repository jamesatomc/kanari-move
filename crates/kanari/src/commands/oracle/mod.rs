// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::cli_types::CommandAction;
use async_trait::async_trait;
use clap::Parser;
use commands::create::CreateCommand;
use commands::reporter::ReporterCommand;
use kanari_types::error::KanariResult;

pub mod commands;

/// Oracle commands
#[derive(Parser)]
pub struct Oracle {
    #[clap(subcommand)]
    cmd: OracleCommand,
}

#[async_trait]
impl CommandAction<String> for Oracle {
    async fn execute(self) -> KanariResult<String> {
        match self.cmd {
            OracleCommand::Create(create) => create.execute_serialized().await,
            OracleCommand::Reporter(server) => server.execute_serialized().await,
        }
    }
}

#[derive(clap::Subcommand)]
#[clap(name = "oracle")]
pub enum OracleCommand {
    Create(CreateCommand),
    Reporter(ReporterCommand),
}
