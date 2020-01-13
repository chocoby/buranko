# buranko

[![Build Status](https://travis-ci.org/chocoby/buranko.svg?branch=master)](https://travis-ci.org/chocoby/buranko)

A tool for parse a git branch name

## Usage

`buranko` prints the `ID` field by default.

```
$ git checkout -b feature/1234_foo-bar
$ buranko
1234
$ buranko -ref
#1234
```

Specify an output field.

```
$ buranko -output Name
foo-bar
```

Parse a branch name from stdin.

```
$ echo 'feature/1234_foo-bar' | buranko
1234
```

## Configuration

Configuration uses 'git-config' variables.

***buranko.reponame***

A repository name.
To output a repository name, use `-reponame` option.

This is useful for commit across the GitHub repository.

```
$ git config buranko.reponame foo-org/bar-repo
$ git checkout -b feature/1234_foo-bar
$ buranko -ref -reponame
foo-org/bar-repo#1234
```


## Fields

* `FullName`: Full branch name
* `Action`: Action type
* `ID`: Issue ID
* `Name`: Name

## Parse patterns

### `feature/1234_foo-bar`

* `FullName`: `feature/1234_foo-bar`
* `Action`: `feature`
* `ID`: `1234`
* `Name`: `foo-bar`

### `foo-bar`

* `FullName`: `foo-bar`
* `Name`: `foo-bar`

More patterns at [`parser_test.go`](https://github.com/chocoby/buranko/blob/master/parser_test.go).

## Integrate with `prepare-commit-msg`

Add an issue ID to commit comment using git hook.

`GIT-REPO/.git/hooks/prepare-commit-msg`

```sh
if [ "$2" == "" ]; then
    mv $1 $1.tmp
    echo `buranko -ref -reponame` > $1
    cat $1.tmp >> $1
fi
```

## Install

To install, use `go get`:

```bash
$ go get github.com/chocoby/buranko
```

Or you can download a binary from [releases page](https://github.com/chocoby/buranko/releases) and place it in `$PATH` directory.

## Contribution

1. Fork ([https://github.com/chocoby/buranko/fork](https://github.com/chocoby/buranko/fork))
1. Create a feature branch
1. Commit your changes
1. Rebase your local changes against the master branch
1. Run test suite with the `go test ./...` command and confirm that it passes
1. Run `gofmt -s`
1. Create a new Pull Request

## License

[MIT License](http://chocoby.mit-license.org)

## Author

[Kenta Okamoto](https://github.com/chocoby)
