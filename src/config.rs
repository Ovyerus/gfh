use std::{
    collections::HashMap,
    error::Error,
    fs::{create_dir_all, read_to_string, write},
    path::Path,
};

type Config = HashMap<String, String>;

pub fn read_config<P: AsRef<Path>>(path: P) -> std::io::Result<Config> {
    let content = read_to_string(path)?;
    let cfg = parse_config(&content);
    Ok(cfg)
}

pub fn write_config<P: AsRef<Path>>(path: P, cfg: Config) -> Result<(), Box<dyn Error>> {
    let serialised = serialise_config(cfg);
    let basepath = path.as_ref().parent().unwrap();
    create_dir_all(basepath)?;
    write(path, serialised)?;
    Ok(())
}

fn parse_config(content: &str) -> Config {
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

    output
}

fn serialise_config(cfg: Config) -> String {
    let mut output = String::new();

    for (serial, path) in cfg {
        output.push_str(&format!("{}::{}\n", serial, path.replace("\\", "\\\\")));
    }

    output
}
