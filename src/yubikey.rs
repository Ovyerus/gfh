use std::error::Error;
use yubikey_api::Context;

use crate::util::FidoDevice;

pub fn get_yubikeys() -> Result<Vec<FidoDevice>, Box<dyn Error>> {
    let mut readers = Context::open()?;
    let mut output = Vec::<FidoDevice>::new();

    for reader in readers.iter()? {
        let yubikey = reader.open()?;
        output.push(FidoDevice::YubiKey(yubikey))
    }

    Ok(output)
}
