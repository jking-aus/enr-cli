//! # enr-cli
//!
//! This is currently a simple tool to read base64 encoded ENRs. More features may be added in
//! the future.
//!
//! ## Install
//!
//! This can be installed via cargo:
//!
//! ```bash
//! $ cargo install enr-cli
//! ```
//!
//! ## Usage
//!
//! ```bash
//! Sigma Prime <contact@sigmaprime.io>
//! Simple CLI for reading and building ENRs.
//!
//! USAGE:
//!     enr-cli [SUBCOMMAND]
//!
//! OPTIONS:
//!     -h, --help       Print help information
//!     -V, --version    Print version information
//!
//! SUBCOMMANDS:
//!     build    Builds an ENR
//!     help     Print this message or the help of the given subcommand(s)
//!     read     Reads an ENR
//!
//!
//! ## Example
//!
//! ```bash
//! $ enr-cli read enr:-Ku4QJsxkOibTc9FXfBWYmcdMAGwH4bnOOFb4BlTHfMdx_f0WN-u4IUqZcQVP9iuEyoxipFs7-Qd_rH_0HfyOQitc7IBh2F0dG5ldHOIAAAAAAAAAACEZXRoMpD1pf1CAAAAAP__________gmlkgnY0gmlwhLAJM9iJc2VjcDI1NmsxoQL2RyM26TKZzqnUsyycHQB4jnyg6Wi79rwLXtaZXty06YN1ZHCCW8w
//! ENR Read:
//! Sequence No:1
//! NodeId:0x3ab5..1447
//! Libp2p PeerId:16Uiu2HAmC13Brucnz5qR8caKi8qKK6766PFoxsF5MzK2RvbTyBRr
//! IP:176.9.51.216
//! UDP Port:23500
//! Known multiaddrs:
//! /ip4/176.9.51.216/udp/23500/p2p/16Uiu2HAmC13Brucnz5qR8caKi8qKK6766PFoxsF5MzK2RvbTyBRr
//! ```

use clap::{Arg, Command};
use enr::{CombinedKey, Enr as EnrRaw};
mod enr_ext;
pub mod eth2_ext;
use ssz::Encode;

use enr_ext::EnrExt;
use eth2_ext::Eth2Enr;

mod build;

pub type Enr = EnrRaw<CombinedKey>;

fn main() {
    // Parse the CLI parameters.
    let matches = Command::new("enr-cli")
        .version("0.2.0")
        .author("Sigma Prime <contact@sigmaprime.io>")
        .about("Simple CLI for reading and building ENRs.")
        .subcommand(read())
        .subcommand(build())
        .get_matches();

    // Handle the read logic
    if let Some(read_matches) = matches.subcommand_matches("read") {
        let enr_base64 = read_matches
            .get_one::<String>("enr")
            .expect("Must supply an ENR");
        let enr = enr_base64.parse::<Enr>().unwrap();
        print_enr(enr);
    } else if let Some(build_matches) = matches.subcommand_matches("build") {
        // Handle the build ENR logic
        if let Err(e) = build::build(build_matches) {
            println!("Failed to build: {}", e);
        }
    }
}

fn read() -> Command {
    Command::new("read").about("Reads an ENR").arg(
        Arg::new("enr")
            .value_name("BASE64-ENR")
            .allow_hyphen_values(true)
            .required(true)
            .help("Reads a base64 ENR and prints common parameters."),
    )
}

fn build() -> Command {
    Command::new("build")
        .about("Builds an ENR")
        .arg(
            Arg::new("private-key")
                .short('k')
                .long("private-key")
                .allow_hyphen_values(true)
                .help("A hex encoded private key to use for signing. If this or --key-file is not specified a random one will be generated")
        )
        .arg(
            Arg::new("key-file")
                .short('j')
                .long("key-file")
                .allow_hyphen_values(true)
                .help("Path to a key file that stores raw bytes of an ENR key. Example for lighthouse is in ~/.lighthouse/mainnet/beacon/network/key.dat.")
        )
        .arg(
            Arg::new("ip")
                .long("ip")
                .short('i')
                .help("Set an IPv4 address")
        )
        .arg(
            Arg::new("ip6")
                .long("ip6")
                .short('I')
                .help("Set an IPv6 address")
                )
        .arg(
            Arg::new("seq")
                .long("seq-no")
                .short('s')
                .help("Set a sequence number")
        )
        .arg(
            Arg::new("tcp-port")
                .long("tcp-port")
                .short('p')
                .help("Set a tcp port")
        )
        .arg(
            Arg::new("tcp6-port")
                .long("tcp6-port")
                .help("Set a TCP port for IPv6")
        )
        .arg(
            Arg::new("udp-port")
                .long("udp-port")
                .short('u')
                .help("Set an udp port")
        )
        .arg(
            Arg::new("udp6-port")
                .long("udp6-port")
                .help("Set a UDP port for IPv6")
        )
        .arg(
            Arg::new("quic-port")
                .long("quic-port")
                .short('q')
                .help("Set a quic port")
        )
        .arg(
            Arg::new("quic6-port")
                .long("quic6-port")
                .short('Q')
                .help("Set an IPv6 QUIC port")
        )
        .arg(
            Arg::new("eth2")
                .long("eth2")
                .short('f')
                .help("Set an eth2 fork field. Takes the raw SSZ bytes input")
        )
}

pub fn print_enr(enr: Enr) {
    println!("ENR Read:");
    println!("Sequence No:{}", enr.seq());
    println!("NodeId: {}", hex::encode(enr.node_id().raw()));
    println!("EnodeId: {}", enr.enode_id());
    println!("Libp2p PeerId: {}", enr.peer_id());
    if let Some(ip) = enr.ip4() {
        println!("ipv4:{:?}", ip);
    }
    if let Some(ip) = enr.ip6() {
        println!("ipv6:{:?}", ip);
    }
    if let Some(tcp) = enr.tcp4() {
        println!("v4_tcp:{}", tcp);
    }
    if let Some(tcp) = enr.tcp6() {
        println!("v6_tcp:{}", tcp);
    }
    if let Some(udp) = enr.udp4() {
        println!("v4_udp:{}", udp);
    }
    if let Some(udp) = enr.udp6() {
        println!("v6_udp:{}", udp);
    }
    if let Some(udp) = enr.quic4() {
        println!("v4_quic:{}", udp);
    }
    if let Some(udp) = enr.quic6() {
        println!("v6_quic:{}", udp);
    }

    if let Ok(enr_fork_id) = enr.eth2() {
        println!(
            "Eth2 Field:\n\tFork digest: {}\n\tNext fork version: {}\n\tNext fork epoch: {}\n\tSSZ Bytes: {}",
            hex::encode(enr_fork_id.fork_digest),
            hex::encode(enr_fork_id.next_fork_version),
            enr_fork_id.next_fork_epoch,
            hex::encode(enr_fork_id.as_ssz_bytes())
        );
    }

    let multiaddrs = enr.multiaddr();
    if !multiaddrs.is_empty() {
        println!("Known multiaddrs:");
        for multiaddr in multiaddrs {
            println!("{}", multiaddr);
        }
    }
}
