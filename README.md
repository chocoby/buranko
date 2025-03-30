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

More patterns at [`main.rs`](https://github.com/chocoby/buranko/blob/main/src/main.rs).

## Integrate with `prepare-commit-msg`

Add an issue ID to the commit message using a git hook.

`[Git repository]/.git/hooks/prepare-commit-msg`

```bash
if [ "$2" == "" ]; then
    mv $1 $1.tmp
    echo `buranko -output LinkID -template` > $1
    cat $1.tmp >> $1
fi
```
