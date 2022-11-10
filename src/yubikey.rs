use std::error::Error;

use yubikey_api::{Context, YubiKey};

pub fn get_yubikeys() -> Result<Vec<YubiKey>, Box<dyn Error>> {
    let mut readers = Context::open()?;
    let mut output = Vec::<YubiKey>::new();

    for reader in readers.iter()? {
        let yubikey = reader.open()?;
        output.push(yubikey)
    }

    Ok(output)
}
