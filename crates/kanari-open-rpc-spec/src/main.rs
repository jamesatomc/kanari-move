// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use clap::ValueEnum;
use pretty_assertions::assert_str_eq;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Parser, Clone, Copy, ValueEnum)]
enum Action {
    Print,
    Test,
    Record,
}

#[derive(Debug, Parser)]
#[clap(name = "Kanari Open RPC Spec", about = "Generate kanari open rpc spec")]
struct Options {
    #[clap(value_enum, default_value = "Record", ignore_case = true)]
    action: Action,
}

#[tokio::main]
async fn main() {
    let options = Options::parse();

    let open_rpc = kanari_open_rpc_spec_builder::build_kanari_rpc_spec();

    match options.action {
        Action::Record => {
            let content = serde_json::to_string_pretty(&open_rpc).unwrap();
            let mut f = File::create(kanari_open_rpc_spec_builder::spec_file()).unwrap();
            writeln!(f, "{content}").unwrap();
        }
        Action::Test => {
            let reference =
                std::fs::read_to_string(kanari_open_rpc_spec_builder::spec_file()).unwrap();
            let content = serde_json::to_string_pretty(&open_rpc).unwrap() + "\n";
            assert_str_eq!(&reference, &content);
        }
        _ => {
            let content = serde_json::to_string_pretty(&open_rpc).unwrap();
            println!("{content}");
        }
    }
}
