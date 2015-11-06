package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"log"
	"os"
	"strings"
	"text/template"

	"regexp"

	pipeline "github.com/mattn/go-pipeline"
)

// A Command is an implementation of a buranko command
type Command struct {
	// Run runs the command.
	// The args are the arguments after the command name.
	Run func(args []string) int

	// UsageLine is the one-line usage message.
	// The first word in the line is taken to be the command name.
	UsageLine string

	// Short is the short description shown in the 'buranko help' output.
	Short string

	// Long is the long message shown in the 'buranko help <this-command>' output.
	Long string

	// Flag is a set of flags specific to this command.
	Flag flag.FlagSet
}

// Name returns the command's name: the first word in the usage line.
func (c *Command) Name() string {
	name := c.UsageLine
	i := strings.Index(name, " ")
	if i >= 0 {
		name = name[:i]
	}
	return name
}

func (c *Command) Usage() {
	fmt.Fprintf(os.Stderr, "usage: %s\n\n", c.UsageLine)
	fmt.Fprintf(os.Stderr, "%s\n", strings.TrimSpace(c.Long))
	os.Exit(2)
}

// Commands lists the available commands and help topics.
// The order here is the order in which they are printed by 'buranko help'.
var commands = []*Command{}

func main() {
	flag.String("output", "id", "Output ticket id")
	flag.Usage = usage
	flag.Parse()
	log.SetFlags(0)

	args := flag.Args()

	if len(args) < 1 {
		doOutput()
		return
	}

	if args[0] == "help" {
		help(args[1:])
		return
	}
}

var usageTemplate = `buranko is a tool for

Usage:

	buranko command [arguments]

The commands are:
{{range .}}
	{{.Name | printf "%-11s"}} {{.Short}}{{end}}

Use "buranko help [command]" for more information about a command.

`

var helpTemplate = `usage: buranko {{.UsageLine}}

{{.Long | trim}}
`

// tmpl executes the given template text on data, writing the result to w.
func tmpl(w io.Writer, text string, data interface{}) {
	t := template.New("top")
	t.Funcs(template.FuncMap{"trim": strings.TrimSpace})
	template.Must(t.Parse(text))
	if err := t.Execute(w, data); err != nil {
		panic(err)
	}
}

func printUsage(w io.Writer) {
	bw := bufio.NewWriter(w)
	tmpl(bw, usageTemplate, commands)
	bw.Flush()
}

func usage() {
	printUsage(os.Stderr)
	os.Exit(2)
}

// help implements the 'help' command.
func help(args []string) {
	if len(args) == 0 {
		printUsage(os.Stdout)
		// not exit 2: succeeded at 'buranko help'.
		return
	}
	if len(args) != 1 {
		fmt.Fprintf(os.Stderr, "usage: buranko help command\n\nToo many arguments given.\n")
		os.Exit(2) // failed at 'buranko help'
	}

	arg := args[0]

	for _, cmd := range commands {
		if cmd.Name() == arg {
			tmpl(os.Stdout, helpTemplate, cmd)
			// not exit 2: succeeded at 'buranko help cmd'.
			return
		}
	}

	fmt.Fprintf(os.Stderr, "Unknown help topic %#q.  Run 'buranko help'.\n", arg)
	os.Exit(2) // failed at 'buranko help cmd'
}

func doOutput() {
	branchName := GetBranchNameFromGitCommand()
	ticketId := Parse(branchName)

	fmt.Println(ticketId)
}

func GetBranchNameFromGitCommand() string {
	out, err := pipeline.Output(
		[]string{"git", "rev-parse", "--abbrev-ref", "HEAD"},
	)

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	return string(out)
}

func Parse(branchName string) string {
	r := regexp.MustCompile(`feature\/(\d+)_.*`)

	matches := r.FindStringSubmatch(branchName)

	if len(matches) == 0 {
		os.Exit(1)
	}

	ticketId := matches[1]

	return ticketId
}
