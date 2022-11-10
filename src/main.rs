use clap::Parser;
// use ctap_hid_fido2::HidInfo;
use expanduser::expanduser;
use std::{error::Error, fs};

mod config;
mod yubikey;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "~/.config/gfh/keys")]
    file: String,
}

// fn get_serial(device: &HidInfo) -> String {
//     let found = device
//         .info
//         .split(' ')
//         .find(|x| x.starts_with("serial_number="));

//     match found {
//         Some(part) => part.split_once('=').unwrap().1.to_owned(),
//         None => {
//             let product = device.product_string.clone();

//             if product.contains("YubiKey") {
//                 // TODO: convert HidInfo -> Yubikey
//                 return String::from("meow");
//             }

//             panic!(
//                 "failed to find serial ID for {product}. Please open an issue with the device name"
//             )
//         }
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = expanduser(args.file)?;
    let content = fs::read_to_string(path).expect("could not read config file");
    let mapping = config::parse_config(&content);

    // Idea: ignore yubikeys from fido listing

    // let devs = ctap_hid_fido2::get_fidokey_devices();
    // if devs.is_empty() {
    //     // TODO: instead of panicing, await a device to be plugged in
    //     panic!("no FIDO devices detected")
    // }

    // for d in devs {
    //     // let serial = get_serial(&d);
    //     println!("{},  {}", d.product_string, d.info);
    // }

    let yubikeys = yubikey::get_yubikeys()?;
    let selected = yubikeys
        .iter()
        .find_map(|y| mapping.get(&y.serial().0.to_string()))
        .expect("no matching FIDO key found in configuration file.");

    // We need to prefix it with `key::` so that Git doesn't reject it. It then
    // gets picked up by `bin/gfh-keygen` which does some magic stuff to feed
    // the key to `ssh-keygen`
    println!("key::{selected}");

    Ok(())
}
