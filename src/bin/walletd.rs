// LNP Node: node running lightning network protocol and generalized lightning
// channels.
// Written in 2020 by
//     Dr. Maxim Orlovsky <orlovsky@pandoracore.com>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the MIT License
// along with this software.
// If not, see <https://opensource.org/licenses/MIT>.

#![recursion_limit = "256"]
// Coding conventions
#![deny(
    non_upper_case_globals,
    non_camel_case_types,
    non_snake_case,
    unused_mut,
    unused_imports,
    dead_code,
    missing_docs
)]

//! Main executable for walletd: farcaster node wallet  microservice.

#[macro_use]
extern crate log;

use clap::Clap;

use farcaster_node::Config;
use farcaster_node::{
    rpc::request::Token,
    walletd::{self, NodeSecrets, Opts},
};

fn main() {
    let mut opts = Opts::parse();
    trace!("Command-line arguments: {:?}", &opts);
    opts.process();
    trace!("Processed arguments: {:?}", &opts);

    let config: Config = opts.shared.clone().into();
    trace!("Daemon configuration: {:?}", &config);
    debug!("MSG RPC socket {}", &config.msg_endpoint);
    debug!("CTL RPC socket {}", &config.ctl_endpoint);

    let wallet_token = Token(opts.token.wallet_token);

    let node_secrets = NodeSecrets::new(opts.key_opts.key_file.clone());
    let node_id = node_secrets.node_id();

    debug!("Starting runtime ...");
    walletd::run(config, wallet_token, node_secrets, node_id)
        .expect("Error running walletd runtime");

    unreachable!()
}
