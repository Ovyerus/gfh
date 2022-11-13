use clap::Parser;
// use ctap_hid_fido2::HidInfo;
use expanduser::expanduser;
use std::{error::Error, fs};

mod add_key;
mod config;
mod util;
mod yubikey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "~/.config/gfh/keys")]
    file: String,

    #[arg(short, long)]
    add: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = expanduser(args.file)?;

    if args.add {
        return add_key::run(path);
    }

    let cfg = config::read_config(path).expect("could not read config path");
    let devices = util::get_all_devices()?;

    let selected = devices
        .iter()
        .find_map(|y| cfg.get(&y.serial()))
        .expect("no matching FIDO key found in configuration file.");

    // TODO: resolve file paths in config relative to the config file?

    // We need to prefix it with `key::` so that Git doesn't reject it. It then
    // gets picked up by `bin/gfh-keygen` which does some magic stuff to feed
    // the key to `ssh-keygen`
    println!("key::{selected}");

    Ok(())
}
