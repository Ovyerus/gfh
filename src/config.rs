use anyhow::{Context, Result};
use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, write},
    path::Path,
};

// use crate::error::{Error, Result};

type Config = HashMap<String, String>;

pub fn read_config<P: AsRef<Path>>(path: P) -> Result<Config> {
    let content = read_to_string(&path).with_context(|| {
        format!(
            "failed to read config at {}",
            path.as_ref().to_string_lossy()
        )
    })?;
    let cfg = parse_config(&content)?;
    Ok(cfg)
}

pub fn write_config<P: AsRef<Path>>(path: P, cfg: Config) -> Result<()> {
    let serialised = serialise_config(cfg);
    let basepath = path.as_ref().parent().unwrap();
    create_dir_all(basepath).with_context(|| {
        format!(
            "failed to create directory tree `{}` for config",
            basepath.to_string_lossy()
        )
    })?;
    write(&path, serialised).with_context(|| {
        format!(
            "failed to write config at {}",
            path.as_ref().to_string_lossy()
        )
    })?;

    Ok(())
}

fn parse_config(content: &str) -> Result<Config> {
    let lines = content.lines().enumerate();
    let mut output = Config::new();

    for (i, line) in lines {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // TODO: function to validate lines & ensure referenced files exist.

        let (serial, key) = line.split_once("::").expect(&format!(
            "malformed line {i} in config. expected `<serial>::<file path>`"
        ));

        output.insert(serial.to_owned(), key.to_owned());
    }

    if output.is_empty() {
        panic!("cannot do anything with an empty config! try using `gfh -a` to import a ssh key");
    }

    Ok(output)
}

fn serialise_config(cfg: Config) -> String {
    let mut output = String::new();

    for (serial, path) in cfg {
        output.push_str(&format!("{}::{}\n", serial, path.replace("\\", "\\\\")));
    }

    output
}
