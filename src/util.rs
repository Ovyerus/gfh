use core::fmt;
use sha2::{Digest, Sha256};
use std::error::Error;

use ctap_hid_fido2::HidInfo;
use yubikey_api::YubiKey;

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

pub fn get_fidos() -> Result<Vec<FidoDevice>, Box<dyn Error>> {
    let devices = ctap_hid_fido2::get_fidokey_devices();
    let devices = devices
        .iter()
        .map(|x| FidoDevice::Generic(x.to_owned()))
        .collect::<Vec<FidoDevice>>();
    Ok(devices)
}

pub fn sha256(input: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let output = hasher.finalize();
    output.to_vec()
}
