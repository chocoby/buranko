package main

import (
	"testing"
)

func TestBranch_LinkID(t *testing.T) {
	branch := NewBranch()
	branch.ID = "1234"
	expected := "#1234"

	if branch.LinkID() != expected {
		t.Fatalf("Expected %v, but %v:", expected, branch.LinkID())
	}
}
