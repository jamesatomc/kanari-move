// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::cli_types::CommandAction;
use crate::commands::transaction::commands::sign_order::SignOrderCommand;
use crate::commands::transaction::commands::{
    build::BuildCommand, get_transactions_by_hash::GetTransactionsByHashCommand,
    get_transactions_by_order::GetTransactionsByOrderCommand, query::QueryCommand,
    sign::SignCommand, submit::SubmitCommand,
};
use async_trait::async_trait;
use clap::{Parser, Subcommand};
use kanari_types::error::KanariResult;

pub mod commands;

/// Tool for interacting with transaction
#[derive(Parser)]
pub struct Transaction {
    #[clap(subcommand)]
    cmd: TransactionCommand,
}

#[async_trait]
impl CommandAction<String> for Transaction {
    async fn execute(self) -> KanariResult<String> {
        match self.cmd {
            TransactionCommand::GetTransactionsByOrder(cmd) => cmd.execute_serialized().await,
            TransactionCommand::GetTransactionsByHash(cmd) => cmd.execute_serialized().await,
            TransactionCommand::Query(cmd) => cmd.execute_serialized().await,
            TransactionCommand::Build(cmd) => cmd.execute_serialized().await,
            TransactionCommand::Sign(cmd) => cmd.execute_serialized().await,
            TransactionCommand::Submit(cmd) => cmd.execute_serialized().await,
            TransactionCommand::SignOrder(cmd) => cmd.execute(),
        }
    }
}

#[derive(Subcommand)]
pub enum TransactionCommand {
    Build(BuildCommand),
    GetTransactionsByOrder(GetTransactionsByOrderCommand),
    GetTransactionsByHash(GetTransactionsByHashCommand),
    Query(QueryCommand),
    Sign(SignCommand),
    Submit(SubmitCommand),
    SignOrder(SignOrderCommand),
}
