package main

var (
	FullName string = ""
	Action   string = ""
	ID       string = ""
	Name     string = ""
)

type Branch struct {
	FullName string
	Action   string
	ID       string
	Name     string
}

func NewBranch() *Branch {
	return &Branch{
		FullName: FullName,
		Action:   Action,
		ID:       ID,
		Name:     Name,
	}
}
