package main

var (
	fullName    string = ""
	action      string = ""
	id          string = ""
	description string = ""
)

// Branch is a branch information
type Branch struct {
	FullName    string
	Action      string
	ID          string
	Description string
}

// NewBranch returns Branch
func NewBranch() *Branch {
	return &Branch{
		FullName:    fullName,
		Action:      action,
		ID:          id,
		Description: description,
	}
}
