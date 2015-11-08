package main

import (
	"bufio"
	"fmt"
	pipeline "github.com/mattn/go-pipeline"
	"os"
)

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
