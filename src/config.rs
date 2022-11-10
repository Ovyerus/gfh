use std::collections::HashMap;

pub fn parse_config(content: &str) -> HashMap<String, String> {
    let lines = content.lines().enumerate();
    let mut output = HashMap::<String, String>::new();

    for (i, line) in lines {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // TODO: function to validate lines & ensure referenced files exist.

        let (serial, key) = line.split_once("::").expect(&format!(
            "malformed line {i} in config. expected `<serial>::<file path>`"
        ));
        // let (keytype, _sshkey) = key
        //     .split_once(" ")
        //     .expect(&format!("malformed ssh key on line {i} in config"));

        // if !keytype.starts_with("sk-ssh") {
        //     panic!("unsupported key type on line {i}: {keytype}");
        // }

        output.insert(serial.to_owned(), key.to_owned());
    }

    if output.is_empty() {
        panic!("cannot do anything with an empty config! try using `gfh -a` to import a ssh key");
    }

    output
}
