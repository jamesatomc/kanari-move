// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, ClientBuilder};
use anyhow::anyhow;
use kanari_config::config::Config;
use kanari_config::server_config::ServerConfig;
use kanari_types::address::KanariAddress;
use kanari_types::kanari_network::BuiltinChainID;
use kanari_types::kanari_network::KanariNetwork;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::{Display, Formatter, Write};
use std::path::PathBuf;

pub const DEFAULT_EXPIRATION_SECS: u64 = 30;
pub const KANARI_DEV_NET_URL: &str = "https://dev-seed.kanari.site";
pub const KANARI_TEST_NET_URL: &str = "https://test-seed.kanari.site";
pub const KANARI_MAIN_NET_URL: &str = "https://main-seed.kanari.site";

#[derive(Serialize, Deserialize, Debug)]
pub struct ClientConfig {
    pub keystore_path: PathBuf,
    pub active_address: Option<KanariAddress>,
    pub envs: Vec<Env>,
    pub active_env: Option<String>,
}

impl ClientConfig {
    pub fn new(keystore_path: PathBuf) -> Self {
        ClientConfig {
            keystore_path,
            active_address: None,
            envs: vec![],
            active_env: None,
        }
    }

    pub fn get_env(&self, alias: &Option<String>) -> Option<&Env> {
        if let Some(alias) = alias {
            self.envs.iter().find(|env| &env.alias == alias)
        } else {
            self.envs.first()
        }
    }

    pub fn get_active_env(&self) -> Result<&Env, anyhow::Error> {
        self.get_env(&self.active_env).ok_or_else(|| {
            anyhow!(
                "Environment configuration not found for env [{}]",
                self.active_env.as_deref().unwrap_or("None")
            )
        })
    }

    pub fn add_env(&mut self, env: Env) {
        let find_env = self
            .envs
            .iter_mut()
            .find(|other_env| other_env.alias == env.alias);
        if let Some(update_env) = find_env {
            update_env.rpc = env.rpc;
            update_env.ws = env.ws;
        } else {
            self.envs.push(env)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Env {
    pub alias: String,
    pub rpc: String,
    pub ws: Option<String>,
}

impl Env {
    pub async fn create_rpc_client(
        &self,
        request_timeout: std::time::Duration,
    ) -> Result<Client, anyhow::Error> {
        let mut builder = ClientBuilder::default();
        builder = builder.request_timeout(request_timeout);
        if let Some(ws_url) = &self.ws {
            builder = builder.ws_url(ws_url);
        }

        builder.build(&self.rpc).await
    }

    pub fn new_dev_env() -> Self {
        Self {
            alias: BuiltinChainID::Dev.chain_name(),
            rpc: KANARI_DEV_NET_URL.into(),
            ws: None,
        }
    }

    pub fn new_test_env() -> Self {
        Self {
            alias: BuiltinChainID::Test.chain_name(),
            rpc: KANARI_TEST_NET_URL.into(),
            ws: None,
        }
    }

    pub fn new_main_env() -> Self {
        Self {
            alias: BuiltinChainID::Main.chain_name(),
            rpc: KANARI_MAIN_NET_URL.into(),
            ws: None,
        }
    }

    /// Guess the network based on the alias for some local use cases, do not want to connec to rpc.
    /// The right way to determine the network is to call the rpc `chain_id` method
    pub fn guess_network(&self) -> KanariNetwork {
        match self.alias.as_str() {
            "dev" => BuiltinChainID::Dev.into(),
            "test" => BuiltinChainID::Test.into(),
            "main" => BuiltinChainID::Main.into(),
            _ => BuiltinChainID::Local.into(),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Env {
            alias: BuiltinChainID::Local.chain_name(),
            rpc: format!("http://127.0.0.1:{}", ServerConfig::default().port),
            ws: None,
        }
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();
        writeln!(writer, "Active environment : {}", self.alias)?;
        write!(writer, "RPC URL: {}", self.rpc)?;
        if let Some(ws) = &self.ws {
            writeln!(writer)?;
            write!(writer, "Websocket URL: {ws}")?;
        }
        write!(f, "{}", writer)
    }
}

impl Config for ClientConfig {}

impl Display for ClientConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut writer = String::new();

        writeln!(writer, "Keystore path : {:?}", self.keystore_path)?;
        write!(writer, "Active address: ")?;
        match self.active_address {
            Some(r) => writeln!(writer, "{}", r)?,
            None => writeln!(writer, "None")?,
        };
        write!(writer, "server: ")?;

        if let Ok(env) = self.get_active_env() {
            write!(writer, "{}", env)?;
        }
        match &self.active_env {
            Some(r) => writeln!(writer, "{}", r)?,
            None => writeln!(writer, "None")?,
        };
        write!(f, "{}", writer)
    }
}
