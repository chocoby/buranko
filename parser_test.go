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
		{`feature/1234_foo-bar`, &Branch{FullName: "feature/1234_foo-bar", Action: "feature", Id: "1234", Name: "foo-bar"}},
		{`feature/1234_foo_bar`, &Branch{FullName: "feature/1234_foo_bar", Action: "feature", Id: "1234", Name: "foo_bar"}},
		{`feature/1234-foo`, &Branch{FullName: "feature/1234-foo", Action: "feature", Id: "1234", Name: "foo"}},
		{`feature/1234`, &Branch{FullName: "feature/1234", Action: "feature", Id: "1234", Name: ""}},
		{`feature/foo`, &Branch{FullName: "feature/foo", Action: "feature", Id: "", Name: "foo"}},
		{`#1234-foo-bar`, &Branch{FullName: "#1234-foo-bar", Action: "", Id: "1234", Name: "foo-bar"}},
		{`foo`, &Branch{FullName: "foo", Action: "", Id: "", Name: "foo"}},
		{`foo-bar`, &Branch{FullName: "foo-bar", Action: "", Id: "", Name: "foo-bar"}},
		{`1234`, &Branch{FullName: "1234", Action: "", Id: "1234", Name: ""}},
		{``, &Branch{FullName: "", Action: "", Id: "", Name: ""}},
	}

	for _, testcase := range testcases {
		parser := Parse(testcase.line)
		if !reflect.DeepEqual(parser, testcase.expected) {
			t.Fatalf("Expected %v, but %v:", testcase.expected, parser)
		}
	}
}
