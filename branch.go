package main

var (
	fullName string = ""
	action   string = ""
	id       string = ""
	name     string = ""
)

// Branch is a branch information
type Branch struct {
	FullName string
	Action   string
	ID       string
	Name     string
}

// NewBranch returns Branch
func NewBranch() *Branch {
	return &Branch{
		FullName: fullName,
		Action:   action,
		ID:       id,
		Name:     name,
	}
}
