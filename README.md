# gfh

> Git FIDO helper, or God Fucking Help me.

gfh is a tool for helping you sign your commits in Git with resident SSH keys
stored on multiple FIDO devices.

<p align="center">
  <a href="#getting-started">Getting Started</a>
  -
  <a href="#usage">Usage</a>
  -
  <a href="#installation">Installation</a>
</p>

**NB:** Currently this project has only been tested with the YubiKey 5C NFC. Any
FIDO2 certified device _should_ be compatible, but please let me know if you
encountered any issues with particular devices, or also if it works fine so that
I can have a running list of all keys that are verified working.

## Getting Started

Before you get started with gfh, you'll need to make sure that you already have
a resident SSH key on your FIDO key(s). The simplest way to do this is via
`ssh-keygen -t ed25519-sk -O resident`, but there are better guides online if
you need some different stuff.

If you don't own multiple FIDO keys/only use one resident SSH key, then you more
than likely do not need to use gfh. This tool has a very niche use case due to
Git not supporting multiple `signingkey`s. If you only use one resident SSH key
for signing your commits, you can just use that config option without gfh.

## Usage

The simplest way to add your keys to gfh is via `gfh -a`. This will prompt you
to select the FIDO key to use, as well as the path to the public key (or private
key) to use with it (this must be a resident key that you generated for that
particular FIDO device).

If you prefer, you can edit the config manually by creating a file at
`~/.config/gfh/keys` with the following format:

```
serial::~/.ssh/id_ed25519_sk
serial::~/.ssh/id_ecdsa_sk
```

(Blank lines & lines starting with `#` will be ignored, but won't be retained if
you use `gfh -a`)

After importing your keys to gfh, run the following commands to set up SSH
signing with Git:

```sh
git config --global commit.gpgsign true
git config --global tag.gpgsign true
git config --global gpg.format "ssh"
git config --global gpg.ssh.program "gfh-keygen"
git config --global gpg.ssh.defaultKeyCommand "gfh"
```

If you're on Windows, change the last two commands to set `gfh-keygen.exe` and
`gfh.exe` respectively.

(You shouldn't set `user.signingkey` because gfh will handle that for you
automatically.)

If all goes according to plan, you should be able to create a new commit or tag
with your FIDO key plugged in, and Git will correctly prompt you to sign with
it.

## Installation

### Releases are currently pending fixed CI builds, in the meantime you can install via Cargo just fine.

Static binary builds of gfh are available on our
[releases page](https://github.com/Ovyerus/gfh/releases) for Windows (x86), Mac
(ARM & x86), and Linux (various architectures).

### Homebrew

`brew install ovyerus/tap/gfh`

### Scoop

```
scoop bucket add ovyerus https://github.com/Ovyerus/bucket
scoop install gfh
```

### Crate

`cargo install gfh`

### From source

Pull this repository and run `cargo build --release`, and look for the `gfh` and
`gfh-keygen` binaries in `./target/release/`.

When building from source or from Cargo, on Linux you will need the following
packages: `pkg-config libpcsclite-dev libudev-dev`

## License

This program is licensed under the MIT license (see [LICENSE](./LICENSE) or
https://opensource.org/licenses/MIT).
