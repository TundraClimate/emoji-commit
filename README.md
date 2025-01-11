# emoji-commit

Create a commit using emojis

# install

```
$ git clone https://github.com/TundraClimate/emoji-commit

$ cd emoji-commit/

$ cargo install --path .
```

# how to use

```
$ ec -h

Create a commit using emojis

Usage: ec [OPTIONS] [PREFIX] [MSG]

Arguments:
  [PREFIX]  Prefix to use
  [MSG]     Commit message

Options:
  -e, --edit                             Edit ec config
  -S, --set-profile <SET_PROFILE>
  -D, --delete-profile <DELETE_PROFILE>
  -L, --list-profile
  -l
  -h, --help                             Print help
  -V, --version                          Print version
```

```sh
# Open config-editor
ec -e
```

```
ec feat "features commit message"
```

## zsh-completion

```
sudo curl https://raw.githubusercontent.com/TundraClimate/emoji-commit/refs/heads/master/_ec -o /usr/local/share/zsh/site-functions/_ec
```
