# Pacman Package Query Tool

A CLI tool to get JSON output of pacman packages from the system's syncdb.

## Usage

Show json output of the pacman package

```
cargo run pacman
```

With the `required_by` field all packages requiring `python-tomli` via {make,check}depends can be found using [jq](https://github.com/jqlang/jq)

```
cargo run python-tomli | jq -r '.[].required_by | join(" ")
```

## Limitations

* `core-testing` and `extra-testing` repositories are not included by default
* `pacquery` expects an up-to-date syncdb and does not provide warning if it is not.

## Completions

Shell completions can be created with `cargo run --bin pacquery_completions` in a
directory specified by the env variable `OUT_DIR`.

## Man page

A man page can be created with `cargo run --bin pacquery_mangen` in a
directory specified by the env variable `OUT_DIR`.
