package main

import (
	"os"
	"regexp"
)

var (
	Id string = ""
)

type Parser struct {
	Id string
}

func NewParser() *Parser {
	return &Parser{Id}
}

func (parser *Parser) Parse(branchName string) *Parser {
	r := regexp.MustCompile(`feature\/(\d+)_.*`)

	matches := r.FindStringSubmatch(branchName)

	if len(matches) == 0 {
		os.Exit(1)
	}

	parser.Id = matches[1]

	return parser
}

func Parse(line string) *Parser {
	return NewParser().Parse(line)
}
