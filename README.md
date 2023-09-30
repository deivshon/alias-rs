# alias-rs

alias-rs translates files with a a unified JSON schema for aliases (described below) to the desired aliases as shell-specific syntax, making its output `eval`-able to configure the desired aliases

## Configuration

If no configuration file is specified through command line arguments, one is searched in the default path `$HOME/.config/alias-rs/config.json`

The configuration file for alias-rs is a JSON file with the following this schema

| Key       | Description                                    | Optional | Default    |
| --------- | ---------------------------------------------- | -------- | ---------- |
| `aliases` | List of alias objects representing the aliases | No       | No aliases |

### Alias objects

| Key      | Description                                                                     | Optional | Default    |
| -------- | ------------------------------------------------------------------------------- | -------- | ---------- |
| `alias`  | Name of the alias                                                               | No       | N/A        |
| `equals` | The aliased string                                                              | No       | N/A        |
| `shells` | A shell object representing on which shells the translation should be performed | Yes      | All shells |

### Shell object

| Key         | Description                                                      | Optional | Default   |
| ----------- | ---------------------------------------------------------------- | -------- | --------- |
| `whitelist` | List of shells where the translation **should** be performed     | Yes $^1$ | No shells |
| `blacklist` | List of shells where the translation **should not** be performed | Yes $^1$ | No shells |

$^1$ Either the `whitelist` or `blacklist` field must be specified if the shell object exists, the fields are mutually exclusive

Refer to `config.example.json` for an example configuration

## Command line arguments

| Argument | Long argument | Description                                             | Optional | Default                              |
| -------- | ------------- | ------------------------------------------------------- | -------- | ------------------------------------ |
| `-c`     | `--config`    | Path of the configuration file                          | Yes      | `$HOME/.config/alias-rs/config.json` |
| `-s`     | `--shell`     | Target shell for which the aliases should be translated | Yes      | `$SHELL`                             |

## Installation

Clone the repository, edit the configuration file and run

```
$ make all && sudo make install
```

The default install path is `/usr/local/bin` and can be changed via the `INSTALL_PATH` environment variable
