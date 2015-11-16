# buranko

A tool for parse a git branch name

## Usage

`buranko` prints the `id` field by default.

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

## Fields

* `FullName`: Full branch name
* `Action`: Action type
* `Id`: Issue id
* `Name`: Name

## Parse patterns

### `feature/1234_foo-bar`

* `FullName`: `feature/1234_foo-bar`
* `Action`: `feature`
* `Id`: `1234`
* `Name`: `foo-bar`

### `foo-bar`

* `FullName`: `foo-bar`
* `Name`: `foo-bar`

More patterns at `parser_test.go`.

## Integrate with `prepare-commit-msg`

Add an issue id to commit comment using git hook.

`GIT-REPO/.git/hooks/prepare-commit-msg`

```sh
if [ "$2" == "" ]; then
    mv $1 $1.tmp
    echo `buranko -ref` > $1
    cat $1.tmp >> $1
fi
```

## Install

To install, use `go get`:

```bash
$ go get -d github.com/chocoby/buranko
```

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
