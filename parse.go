package main

import (
	"regexp"
)

func Parse(fullName string) *Branch {
	r := regexp.MustCompile(`(\S+)\/(\d+)_(\S+)`)

	matches := r.FindStringSubmatch(fullName)

	branch := NewBranch()

	if len(matches) == 0 {
		return branch
	}

	branch.FullName = fullName
	branch.Action = matches[1]
	branch.Id = matches[2]
	branch.Name = matches[3]

	return branch
}
