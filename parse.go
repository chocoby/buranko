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

func (p *Parser) Parse(branchName string) string {
	r := regexp.MustCompile(`feature\/(\d+)_.*`)

	matches := r.FindStringSubmatch(branchName)

	if len(matches) == 0 {
		os.Exit(1)
	}

	ticketId := matches[1]

	return ticketId
}

func Parse(line string) string {
	return NewParser().Parse(line)
}
