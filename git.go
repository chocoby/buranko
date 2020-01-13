package main

import (
	"strings"

	pipeline "github.com/mattn/go-pipeline"
)

// GetRepoName returns a configured repository name.
func GetRepoName() string {
	out, err := pipeline.Output(
		[]string{"git", "config", "--get", "buranko.reponame"},
	)

	if err != nil {
		return ""
	}

	return strings.TrimRight(string(out), "\n")
}
