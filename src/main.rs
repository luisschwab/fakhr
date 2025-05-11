//! Fakhr (فخر - meaning pride in arabic) is a vanity Bitcoin address and Nostr key generator.
//!
//! Supports all Bitcoin script types and networks.
//!
//! The Vanity Address™ has two parts:
//!   1. Prefix: This determines the script type and the network.
//!   2. Suffix: This is the vanity + random parts.

use std::time::Instant;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::sync::mpsc;

use anyhow::{Result, anyhow};
use bitcoin::key::UntweakedPublicKey;
use bitcoin::secp256k1::{Keypair, Secp256k1, rand};
use bitcoin::{Address, AddressType, CompressedPublicKey, Network, PrivateKey, PublicKey, Script};
use clap::builder::PossibleValuesParser;
use clap::{Parser, command};
use nostr::nips::nip19::ToBech32;
use nostr::Keys;
use num_format::{Locale, ToFormattedString};

const BASE58_SET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
const BECH32_SET: &str = "qpzry9x8gf2tvdw0s3jn54khce6mua7l";

#[derive(Parser, Debug)]
#[command(author="luisschwab", version, about="Fakhr (فخر - meaning pride in arabic) is a vanity Bitcoin address and Nostr key generator.", long_about = None)]
struct Args {
    #[arg(short, long, value_parser=PossibleValuesParser::new(["1", "3", "bc1q", "bc1p", "m", "n", "2", "tb1q", "tb1p", "npub1"]))]
    prefix: String,

    #[arg(short, long)]
    suffix: String,
}

/// This checks that the suffix only contains characters present in the charset defined by the
/// prefix, and returns the corresponding ([`AddressType`], [`Network`]) tuple if there are no conflicts.
fn parse_bitcoin_prefix_suffix(prefix: String, suffix: String) -> Result<(AddressType, Network)> {
    // Assemble address type and network tuples
    let (address_type, network) = match prefix.as_str() {
        // Mainnet
        "1" => (AddressType::P2pkh, Network::Bitcoin),
        "3" => (AddressType::P2sh, Network::Bitcoin),
        "bc1q" => (AddressType::P2wpkh, Network::Bitcoin),
        //"bc1q" => (AddressType::P2wsh, Network::Bitcoin),
        "bc1p" => (AddressType::P2tr, Network::Bitcoin),

        // Test Networks
        "m" => (AddressType::P2pkh, Network::Testnet),
        "n" => (AddressType::P2pkh, Network::Testnet),
        "2" => (AddressType::P2sh, Network::Testnet),
        "tb1q" => (AddressType::P2wpkh, Network::Testnet),
        //"tb1q" => (AddressType::P2wsh, Network::Testnet),
        "tb1p" => (AddressType::P2tr, Network::Testnet),

        // Clap won't allow invalid prefixes.
        _ => unreachable!(),
    };

    // Verify that the suffix does not contain
    // any characters outside of the charset defined by the prefix.
    if matches!(address_type, AddressType::P2pkh | AddressType::P2sh) {
        if let Some(invalid) = suffix.chars().find(|&c| !BASE58_SET.contains(c)) {
            return Err(anyhow!("Invalid character for base58 charset: {}", invalid));
        }
    }
    if matches!(address_type, AddressType::P2wpkh | AddressType::P2wsh | AddressType::P2tr) {
        if let Some(invalid) = suffix.chars().find(|&c| !BECH32_SET.contains(c)) {
            return Err(anyhow!("Invalid character for bech32 charset: {}", invalid));
        }
    }

    Ok((address_type, network))
}

/// Mine a Bitcoin address with a given suffix.
fn mine_bitcoin(prefix: String, suffix: String, address_type: AddressType, network: Network) {
    let mined = Arc::new(AtomicBool::new(false));
    let n_threads = num_cpus::get() - 2; // leave some for the rest of us
    let (sender, receiver) = mpsc::channel::<(String, String, String)>();

    let mut handles = vec![];
    // Total iterations across all threads
    let mut total_iterations: u128 = 0;

    let start = Instant::now();
    for _ in 0..n_threads {
        let prefix = prefix.clone();
        let suffix = suffix.clone();
        let mined = mined.clone();
        let sender = sender.clone();

        let handle = thread::spawn(move || {
            let mut iter: u128 = 0;
            let secp = Secp256k1::new();

            while !mined.load(Ordering::Relaxed) {
                let (secretkey, pubkey) = secp.generate_keypair(&mut rand::thread_rng());

                // convert from secp256k1::PublicKey into bitcoin::PublicKey
                let pubkey = PublicKey::new(pubkey);
                let privkey = PrivateKey {
                    compressed: true,
                    network: network.into(),
                    inner: secretkey,
                };

                let address = match address_type {
                    AddressType::P2pkh => Address::p2pkh(pubkey, network),
                    AddressType::P2sh => {
                        let redeem_script =
                            Script::builder().push_key(&pubkey).push_opcode(bitcoin::opcodes::all::OP_CHECKSIG).into_script();

                        Address::p2sh(&redeem_script, network).unwrap()
                    }
                    AddressType::P2wpkh => {
                        let compressed_pubkey = CompressedPublicKey::from_private_key(&secp, &privkey)
                            .expect("failed to construct a compressed pubkey!");

                        Address::p2wpkh(&compressed_pubkey, network)
                    }
                    AddressType::P2tr => {
                        let keypair = Keypair::from_secret_key(&secp, &secretkey.into());
                        let (x_only_pubkey, _parity) = UntweakedPublicKey::from_keypair(&keypair);

                        Address::p2tr(&secp, /*internal_key=*/ x_only_pubkey, /*merkle_root=*/ None, network)
                    }
                    _ => unreachable!(),
                };

                // Log every 1000 addresses so mining is not bottenecked by I/O
                if iter % 1000 == 0 {
                    println!("{}", address.to_string());
                }

                // Check if address contains the suffix at the beginning.
                let address_string = address.to_string();
                if let Some(sans_prefix) = address_string.strip_prefix(&prefix) {
                    if sans_prefix.to_string().starts_with(&suffix) {
                        println!("{}", address.to_string());

                        mined.store(true, Ordering::Relaxed);
                        sender.send((address_string, secretkey.display_secret().to_string(), privkey.to_wif())).expect("sender failed to send");

                        break;
                    }
                }

                iter += 1;
            }

            iter
        });

        handles.push(handle);
    }

    // Drop the original sender so
    // receiver.recv() knows that the sender is terminated
    drop(sender);

    match receiver.recv() {
        Ok((address_string, secret_key, wif)) => {
            for handle in handles {
                if let Ok(iterations) = handle.join() {
                    total_iterations += iterations;
                }
            }

            let elapsed = start.elapsed().as_secs() as u128 | 1;

            println!(
                "\nFound {} in {:02}:{:02}:{:02} and {} iterations ({} iter/s)",
                address_string,
                elapsed / 3600,
                elapsed % 3600 / 60,
                elapsed % 60,
                total_iterations.to_formatted_string(&Locale::en),
                (total_iterations / elapsed as u128).to_formatted_string(&Locale::en)
            );
            println!("SecretKey: {}", secret_key);
            println!("WIF: {}", wif);
        },

        Err(_) => panic!("threads were terminated before finding a match"),
    }
}

/// This checks that the suffix only contains characters present in the BECH32_SET.
fn parse_nostr_suffix(suffix: String) -> Result<()> {
    if let Some(invalid) = suffix.chars().find(|&c| !BECH32_SET.contains(c)) {
        return Err(anyhow!("Invalid character for bech32 charset: {}", invalid));
    }

    Ok(())
}

// Mine a nostr pubkey with a given suffix.
fn mine_nostr(prefix: String, suffix: String) {
    let mined = Arc::new(AtomicBool::new(false));
    let n_threads = num_cpus::get() - 2; // leave some for the rest of us
    let (sender, receiver) = mpsc::channel::<(String, String)>();

    let mut handles = vec![];
    // Total iterations across all threads
    let mut total_iterations: u128 = 0;

    let start = Instant::now();
    for _ in 0..n_threads {
        let prefix = prefix.clone();
        let suffix = suffix.clone();
        let mined = mined.clone();
        let sender = sender.clone();

        let handle = thread::spawn(move || {
            let mut iter: u128 = 0;

            while !mined.load(Ordering::Relaxed) {
                let keys = Keys::generate();

                let pubkey = keys.public_key().to_bech32().expect("failed to get bech32-encoded nostr pubkey");
                let pubkey_clone = pubkey.clone();

                // Log every 1000 keys so mining is not bottenecked by I/O
                if iter % 1000 == 0 {
                    println!("{}", pubkey.to_string());
                }

                let sans_prefix = pubkey_clone.strip_prefix(&prefix).expect("failed to strip prefix from bech32-encoded nostr pubkey");

                if sans_prefix.starts_with(&suffix) {
                    println!("{}", pubkey.to_string());

                    let privkey = keys.secret_key().to_bech32().expect("failed to get bech32-encoded nostr privkey");
                    mined.store(true, Ordering::Relaxed);
                    sender.send((pubkey, privkey)).expect("sender failed to send");

                    break;
                }

                iter += 1;
            }

            iter
        });

        handles.push(handle);
    }

    // Drop the original sender so
    // receiver.recv() knows that the sender is terminated
    drop(sender);

    match receiver.recv() {
        Ok((pubkey, privkey)) => {
            for handle in handles {
                if let Ok(iterations) = handle.join() {
                    total_iterations += iterations;
                }
            }

            let elapsed = start.elapsed().as_secs() as u128 | 1;

            println!(
                "\nFound {} in {:02}:{:02}:{:02} and {} iterations ({} iter/s)",
                pubkey,
                elapsed / 3600,
                elapsed % 3600 / 60,
                elapsed % 60,
                total_iterations.to_formatted_string(&Locale::en),
                (total_iterations / elapsed as u128).to_formatted_string(&Locale::en)
            );
            println!("SecretKey: {}", privkey);
        },

        Err(_) => panic!("threads were terminated before finding a match"),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let prefix = args.prefix;
    let suffix = args.suffix;

    if prefix == "npub1" {
        parse_nostr_suffix(suffix.clone())?;

        mine_nostr(prefix.clone(), suffix.clone());
    } else {
        let (address_type, network) = parse_bitcoin_prefix_suffix(prefix.clone(), suffix.clone())?;

        mine_bitcoin(prefix, suffix, address_type, network);
    }

    Ok(())
}
