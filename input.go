package main

import (
	"bufio"
	"fmt"
	"os"

	pipeline "github.com/mattn/go-pipeline"
)

// GetBranchNameFromStdin returns branch name from stdin.
func GetBranchNameFromStdin() string {
	out := ""

	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		out = scanner.Text()
		break
	}

	if err := scanner.Err(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	return out
}

// GetBranchNameFromGitCommand returns branch name from git command.
func GetBranchNameFromGitCommand() string {
	out, err := pipeline.Output(
		[]string{"git", "rev-parse", "--abbrev-ref", "HEAD"},
	)

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	return string(out)
}
