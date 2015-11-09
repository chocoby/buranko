package main

import (
	"reflect"
	"testing"
)

func TestParse(t *testing.T) {
	testcases := []struct {
		line     string
		expected *Parser
	}{
		{`feature/1234_foo`, &Parser{Id: "1234"}},
	}

	for _, testcase := range testcases {
		parser := Parse(testcase.line)
		if !reflect.DeepEqual(parser, testcase.expected) {
			t.Fatalf("Expected %v, but %v:", testcase.expected, parser)
		}
	}
}
