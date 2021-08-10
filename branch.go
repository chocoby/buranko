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

// LinkID returns ID with a leading #
// This can be used to link the issue ID to the commit message
func (b *Branch) LinkID() string {
	return "#" + b.ID
}
