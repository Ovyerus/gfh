use anyhow::{anyhow, Context, Result};
use ctap_hid_fido2::{util::to_hex_str, Cfg, FidoKeyHidFactory};
use inquire::{Select, Text};
use shellexpand::tilde;
// use osshkeys::PublicKey;
use std::{collections::HashMap, fmt::Display, fs, path::Path};

use crate::{
    config,
    util::{self, FidoDevice},
    yubikey,
};

struct FidoDescriptorAndEntity {
    id: Vec<u8>,
    name: String,
    display_name: String,
}

impl Display for FidoDescriptorAndEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}... {} ({})",
            &to_hex_str(&self.id)[..7],
            self.display_name,
            self.name
        )
    }
}

pub fn run<P: AsRef<Path>>(path: P) -> Result<()> {
    let keys = util::get_all_devices()?;

    if keys.is_empty() {
        return Err(anyhow!("was not able to find any connected FIDO keys"));
    }

    // TODO: wrapper fn that returns the sole result if len == 1?
    let key = Select::new("Select which key to import:", keys)
        .prompt()
        .with_context(|| "failed to create key selection input")?;
    let ssh_key = loop {
        let answer = Text::new("Path to the associated SSH key:")
            .prompt()
            .with_context(|| "failed to create key path text input")?;
        let expanded = tilde(&answer).into_owned();

        match fs::File::open(&expanded) {
            Ok(_) => {
                // Using expanded here since canonicalize does not like `~`. Maybe manually replace the home path with `~` in user input?
                let tmp = fs::canonicalize(expanded).unwrap();
                break tmp.to_str().unwrap().to_owned();
            }
            Err(_) => println!("Failed to read file. Try something else."),
        }
    };

    // let mut cfg = config::read_config(&path).or_else(|e| match e.kind() {
    //     std::io::ErrorKind::NotFound => Ok(HashMap::new()),
    //     _ => Err(Error::ConfigRead),
    // })?;
    let mut cfg =
        config::read_config(&path).or_else(|e| match e.downcast_ref::<std::io::Error>() {
            None => Err(e),
            Some(inner) => match inner.kind() {
                std::io::ErrorKind::NotFound => Ok(HashMap::new()),
                _ => Err(e),
            },
        })?;

    cfg.insert(key.serial(), ssh_key);
    config::write_config(path, cfg).unwrap();
    println!("Success!");

    // let pin = Text::new("Input your key's PIN:").prompt()?;

    // ** Until ED25519 keys get fixed in ctap-hid-fido2/I find a good way to
    //    pull it from a Yubikey, this auto import feature will need to be on hold.

    // let ssh_key: String = match answer {
    //     FidoDevice::YubiKey(yubi) => String::from("yubi yubi"),
    //     FidoDevice::Generic(device) => {
    //         let rpid = util::sha256("ssh:");
    //         let fidokey = FidoKeyHidFactory::create_by_params(&vec![device.param], &Cfg::init())?;
    //         let creds =
    //             fidokey.credential_management_enumerate_credentials(Some(&pin), rpid.as_slice())?;

    //         let render_options = creds
    //             .iter()
    //             .map(|c| FidoDescriptorAndEntity {
    //                 id: c.public_key_credential_descriptor.id.clone(),
    //                 name: c.public_key_credential_user_entity.name.clone(),
    //                 display_name: c.public_key_credential_user_entity.display_name.clone(),
    //             })
    //             .collect::<Vec<FidoDescriptorAndEntity>>();

    //         let descriptor =
    //             Select::new("Select a SSH key from your device:", render_options).prompt()?;
    //         let cred = creds
    //             .iter()
    //             .find(|c| c.public_key_credential_descriptor.id == descriptor.id)
    //             .unwrap();

    //         // PROBLEM: ed25519 results in blank der & pem. Need to pr to try fix it if possible.
    //         if cred.public_key.pem.is_empty() {
    //             panic!("Public key PEM is empty. Probably means that the SSH key is ED25519 which is currently broken for this. Will fix soon!");
    //         }

    //         let x = PublicKey::from_keystr(&cred.public_key.pem)?;
    //         let ssh_key = x.serialize()?;

    //         ssh_key
    //     }
    // };

    Ok(())
}
