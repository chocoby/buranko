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
		{`feature/1234_foo`, &Branch{FullName: "feature/1234_foo", Action: "feature", ID: "1234", Description: "foo"}},
		{`feature/1234_foo-bar`, &Branch{FullName: "feature/1234_foo-bar", Action: "feature", ID: "1234", Description: "foo-bar"}},
		{`feature/1234_foo_bar`, &Branch{FullName: "feature/1234_foo_bar", Action: "feature", ID: "1234", Description: "foo_bar"}},
		{`feature/1234-foo`, &Branch{FullName: "feature/1234-foo", Action: "feature", ID: "1234", Description: "foo"}},
		{`feature/1234`, &Branch{FullName: "feature/1234", Action: "feature", ID: "1234", Description: ""}},
		{`feature/foo`, &Branch{FullName: "feature/foo", Action: "feature", ID: "", Description: "foo"}},
		{`#1234-foo-bar`, &Branch{FullName: "#1234-foo-bar", Action: "", ID: "1234", Description: "foo-bar"}},
		{`JRA-1234`, &Branch{FullName: "JRA-1234", Action: "", ID: "1234", Description: ""}},
		{`foo`, &Branch{FullName: "foo", Action: "", ID: "", Description: "foo"}},
		{`foo-bar`, &Branch{FullName: "foo-bar", Action: "", ID: "", Description: "foo-bar"}},
		{`1234`, &Branch{FullName: "1234", Action: "", ID: "1234", Description: ""}},
		{``, &Branch{FullName: "", Action: "", ID: "", Description: ""}},
	}

	for _, testcase := range testcases {
		parser := Parse(testcase.line)
		if !reflect.DeepEqual(parser, testcase.expected) {
			t.Fatalf("Expected %v, but %v:", testcase.expected, parser)
		}
	}
}
