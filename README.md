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
  -e, --edit     Edit ec config
  -h, --help     Print help
  -V, --version  Print version
```

```sh
# Open config-editor
ec -e
```

```
ec feat "features commit message"
```
