package main

import (
	"strings"

	pipeline "github.com/mattn/go-pipeline"
)

// GetTemplate returns a configured template.
func GetTemplate() string {
	out, err := pipeline.Output(
		[]string{"git", "config", "--get", "buranko.template"},
	)

	if err != nil {
		return ""
	}

	return strings.TrimRight(string(out), "\n")
}
