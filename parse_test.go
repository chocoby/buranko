package main

import (
	"reflect"
	"testing"
)

func TestParse(t *testing.T) {
	testcases := []struct {
		line     string
		expected *Branch
	}{
		{`feature/1234_foo`, &Branch{FullName: "feature/1234_foo", Action: "feature", Id: "1234", Name: "foo"}},
	}

	for _, testcase := range testcases {
		parser := Parse(testcase.line)
		if !reflect.DeepEqual(parser, testcase.expected) {
			t.Fatalf("Expected %v, but %v:", testcase.expected, parser)
		}
	}
}
