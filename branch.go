package main

var (
	FullName string = ""
	Action   string = ""
	Id       string = ""
	Name     string = ""
)

type Branch struct {
	FullName string
	Action   string
	Id       string
	Name     string
}

func NewBranch() *Branch {
	return &Branch{
		FullName: FullName,
		Action:   Action,
		Id:       Id,
		Name:     Name,
	}
}
