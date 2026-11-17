# Buranko

![CI Status](https://github.com/chocoby/buranko/actions/workflows/ci.yml/badge.svg)

Buranko is a CLI tool for parsing and formatting Git branch names.

## Features

- Git branch name parsing
- Output formatting using templates
- Field extraction

## Installation

Download a binary from [releases page](https://github.com/chocoby/buranko/releases) and place it in `$PATH` directory.

## Usage

### Basic Usage

Display the current branch ID:

```bash
buranko
```

### Options

#### `-t, --template`

Format the output using a configured template:

```bash
buranko --template
```

Template configuration:

```bash
git config buranko.template "{Action}/{ID}-{Description}"
```

Available template variables:

- `{FullName}`: Full branch name
- `{Action}`: Action (feature, bugfix, etc.)
- `{ID}`: Branch ID
- `{LinkID}`: Link ID (with #)
- `{Description}`: Branch description

This option is useful for referencing issues in other GitHub repositories, or for software that requires a prefix in the issue number.

```bash
$ git checkout -b feature/1234_foo-bar
$ git config buranko.template ABC-{ID}
$ buranko --template
ABC-1234
$ git config buranko.template foo-org/bar-repo#{ID}
$ buranko --template
foo-org/bar-repo#1234
```

#### `--output <field>`

Output only a specific field:

```bash
buranko --output FullName
```

Available fields:

- `FullName`: Full branch name
- `Action`: Action
- `ID`: Branch ID
- `LinkID`: Link ID
- `Description`: Branch description

#### `-v, --verbose`

Display all branch information:

```bash
buranko --verbose
```

### Input from Pipe

You can read branch names from stdin:

```bash
echo "feature/123-test" | buranko
```

## Branch Name Format

Buranko parses branch names in the following format:

```
<action>/<id>-<description>
```

Examples:
- `feature/123-add-login`
- `bugfix/456-fix-crash`
- `hotfix/789-security-patch`

More patterns at [`branch.rs`](src/branch.rs).

## Integrate with `prepare-commit-msg`

Add an issue ID to the commit message using a git hook.

`[Git repository]/.git/hooks/prepare-commit-msg`

```bash
#!/bin/sh
if [ -z "$2" ]; then
    issue=$(buranko --output LinkID) || exit 1
    if [ -n "$issue" ] && [ "$issue" != "#" ]; then
        { printf '%s\n' "$issue"; cat "$1"; } > "$1.tmp" || exit 1
        mv "$1.tmp" "$1"
    fi
fi
```

For configured issue prefixes, replace `buranko --output LinkID` in the hook
with `buranko --template`.

## Development

Install [mise](https://mise.jdx.dev/), then run:

```bash
mise install
mise exec -- cargo test --locked
mise exec -- cargo fmt --all -- --check
mise exec -- cargo clippy --locked --all-targets -- -D warnings
```

`mise.toml` pins Rust 1.100.0. CI also tests the latest stable Rust.
Linux releases use the matching muslrust image; macOS releases target both Apple Silicon
(`aarch64-apple-darwin`) and Intel (`x86_64-apple-darwin`), and Windows releases target x64.

Empty stdin falls back to the current Git branch. Non-empty piped input is parsed
as a branch name. Runtime errors and missing templates are reported on stderr
with a non-zero exit status.
