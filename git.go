package main

import (
	pipeline "github.com/mattn/go-pipeline"
	"strings"
)

func GetRepoName() string {
	out, err := pipeline.Output(
		[]string{"git", "config", "--get", "buranko.reponame"},
	)

	if err != nil {
		return ""
	}

	return strings.TrimRight(string(out), "\n")
}
