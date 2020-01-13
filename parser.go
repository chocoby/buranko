package main

import (
	"regexp"
)

func Parse(fullName string) *Branch {
	branch := NewBranch()

	re := regexp.MustCompile(`(\S+)\/(\d+)_(\S+)`)
	matches := re.FindStringSubmatch(fullName)

	if len(matches) > 0 {
		branch.FullName = fullName
		branch.Action = matches[1]
		branch.ID = matches[2]
		branch.Name = matches[3]

		return branch
	}

	re = regexp.MustCompile(`(\S+)\/(\d+)-(\S+)`)
	matches = re.FindStringSubmatch(fullName)

	if len(matches) > 0 {
		branch.FullName = fullName
		branch.Action = matches[1]
		branch.ID = matches[2]
		branch.Name = matches[3]

		return branch
	}

	re = regexp.MustCompile(`(\S+)\/(\d+)`)
	matches = re.FindStringSubmatch(fullName)

	if len(matches) > 0 {
		branch.FullName = fullName
		branch.Action = matches[1]
		branch.ID = matches[2]

		return branch
	}

	re = regexp.MustCompile(`(\S+)\/(\S+)`)
	matches = re.FindStringSubmatch(fullName)

	if len(matches) > 0 {
		branch.FullName = fullName
		branch.Action = matches[1]
		branch.Name = matches[2]

		return branch
	}

	re = regexp.MustCompile(`#(\d+)-(\S+)`)
	matches = re.FindStringSubmatch(fullName)

	if len(matches) > 0 {
		branch.FullName = fullName
		branch.ID = matches[1]
		branch.Name = matches[2]

		return branch
	}

	re = regexp.MustCompile(`(\d+)`)
	matches = re.FindStringSubmatch(fullName)

	if len(matches) > 0 {
		branch.FullName = fullName
		branch.ID = matches[1]

		return branch
	}

	re = regexp.MustCompile(`(\S+)`)
	matches = re.FindStringSubmatch(fullName)

	if len(matches) > 0 {
		branch.FullName = fullName
		branch.Name = matches[1]

		return branch
	}

	return branch
}
