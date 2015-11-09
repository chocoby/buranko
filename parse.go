package main

import (
	"os"
	"regexp"
)

func Parse(fullName string) *Branch {
	r := regexp.MustCompile(`feature\/(\d+)_.*`)

	matches := r.FindStringSubmatch(fullName)

	if len(matches) == 0 {
		os.Exit(1)
	}

	return &Branch{
		FullName: fullName,
		Id:       matches[1],
	}
}
