use anyhow::Result;
use core::fmt;
use ctap_hid_fido2::HidInfo;
use sha2::{Digest, Sha256};
use std::hint::unreachable_unchecked;
use yubikey_api::YubiKey;

use crate::yubikey;

pub enum FidoDevice {
    YubiKey(YubiKey),
    Generic(HidInfo),
}

impl FidoDevice {
    pub fn name(&self) -> String {
        match self {
            Self::YubiKey(yubi) => yubi.name().to_owned(),
            Self::Generic(device) => device.product_string.clone(),
        }
    }

    pub fn serial(&self) -> String {
        match self {
            Self::YubiKey(yubi) => yubi.serial().0.to_string(),
            Self::Generic(device) => {
                let found = device
                    .info
                    .split(' ')
                    .find(|x| x.starts_with("serial_number="));

                match found {
                    Some(part) => part.split_once('=').unwrap().1.to_owned(),
                    None => String::from("unknown"), // None => panic!("failed to find serial ID for {}. Please open an issue so we can try to resolve this!", self.name()),
                }
            }
        }
    }
}

impl fmt::Display for FidoDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.serial(), self.name())
    }
}

pub fn get_generics() -> Vec<FidoDevice> {
    let devices = ctap_hid_fido2::get_fidokey_devices();
    let devices = devices
        .iter()
        .map(|x| FidoDevice::Generic(x.to_owned()))
        .collect::<Vec<FidoDevice>>();
    devices
}

pub fn get_all_devices() -> Result<Vec<FidoDevice>> {
    let fidos = get_generics();
    let mut yubikeys = yubikey::get_yubikeys()?;
    let mut fidos = fidos
        .into_iter()
        .filter(|v| matches!(v, FidoDevice::Generic(_)))
        .filter(|x| match x {
            FidoDevice::Generic(h) => !h.product_string.to_lowercase().contains("yubikey"),
            // Literally only FidoDevice::Generic can be in this vec.
            _ => unsafe { unreachable_unchecked() },
        })
        .collect::<Vec<FidoDevice>>();

    fidos.append(&mut yubikeys);
    Ok(fidos)
}

#[allow(dead_code)]
pub fn sha256(input: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let output = hasher.finalize();
    output.to_vec()
}
