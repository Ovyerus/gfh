# gfh

> Git FIDO helper, or God Fucking Help me.

gfh is a tool for helping you sign your commits in Git with resident SSH keys
stored on multiple FIDO devices.

**NB:** Right now this library only supports YubiKeys due to that being the only
FIDO devices I have, however the end goal of this tool is to be fully compatible
with all FIDO2 compatible keys. 1.0 won't come until this result is achieved.

<!-- Currently this project has only been tested with the YubiKey 5C NFC. If you use
this with different FIDO devices, please let me know of your experience and if
you encountered any issues. -->

<!-- ## Getting Started -->

<!-- smth about how to generate a resident key -->
<!-- Before you get started with gfh, you'll need to make sure that -->

## Usage

Fill out a file called `~/.config/gfh/keys` with the following format on each
line (blank lines & lines starting with `#` will be ignored):

```
serial::~/.ssh/id_ed25519_sk
serial::~/.ssh/id_ecdsa_sk
```

In the future, gfh will provide a option/command to automatically pull this from
your FIDO key.

Next, run the following commands to set up SSH signing with Git:

```sh
git config --global commit.gpgsign true
git config --global tag.gpgsign true
git config --global gpg.format "ssh"
git config --global gpg.ssh.program "gfh-keygen"
git config --global gpg.ssh.defaultKeyCommand "gfh"
```

(You shouldn't set `user.signingkey` because gfh will handle that for you
automatically.)

If all goes according to plan, you should be able to create a new commit or tag
with your FIDO key plugged in, and Git will correctly prompt you to sign with
it.
