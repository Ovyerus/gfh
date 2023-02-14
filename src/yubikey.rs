use anyhow::{Context, Result};
use yubikey_api::Context as YKContext;

use crate::util::FidoDevice;

pub fn get_yubikeys() -> Result<Vec<FidoDevice>> {
    let mut readers: YKContext =
        YKContext::open().with_context(|| "failed to create reader context for yubikeys")?;
    let mut output = Vec::<FidoDevice>::new();

    for reader in readers.iter()? {
        if reader.name().as_ref().to_ascii_lowercase().contains(&"yubikey") {
            let yubikey = reader
                .open()
                .with_context(|| format!("failed to open yubikey {}", reader.name()))?;
            output.push(FidoDevice::YubiKey(yubikey))   
        }
    }

    Ok(output)
}
