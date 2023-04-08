//! This module defines structs for serde based
//! deserialization of the configuration
//!
//! The hierarchy is:
//!
//! `Config > Account > Certificate`
use serde::Deserialize;
use std::{net::SocketAddr, path::PathBuf};

/// Entry-point of the module
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// One listening address per config
    // Legacy misspelled alias. TODO: remove in a future version
    #[serde(alias = "dns_listen_adr")]
    pub dns_listen_addr: SocketAddr,
    /// Several accounts per config
    pub accounts: Vec<Account>,
}

/// Config item representing an ACME account
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Account {
    pub email: String,
    pub private_key_path: PathBuf,
    pub certificates: Vec<Certificate>,
}

/// Config item representing an ACME certificate
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Certificate {
    pub domains: Vec<String>,
    pub fullchain_output_file: PathBuf,
    pub key_output_file: PathBuf,
}
