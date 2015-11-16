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
)

const Version string = "0.1.0"

var (
	output string
	ref    bool
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
	flag.StringVar(&output, "output", "Id", "Output field")
	flag.BoolVar(&ref, "ref", false, "Add reference mark")
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

	if args[0] == "version" {
		version()
		return
	}
}

var usageTemplate = `buranko is a tool for parse a git branch name

Usage:

    buranko commands [arguments]

The commands are:
    help        Show this help
    version     Output version information

Options:
    -output
        Specify an output field.
        Available fields are FullName, Action, Id, Name.

    -ref
        Add reference mark (#) when output id field.
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

func version() {
	fmt.Fprintf(os.Stdout, "branko version v%s\n", Version)
}

func doOutput() {
	branchName := ""

	stat, _ := os.Stdin.Stat()
	if (stat.Mode() & os.ModeCharDevice) == 0 {
		branchName = GetBranchNameFromStdin()
	} else {
		branchName = GetBranchNameFromGitCommand()
	}

	branch := Parse(branchName)

	switch output {
	case "FullName":
		fmt.Print(branch.FullName)
	case "Action":
		fmt.Print(branch.Action)
	case "Id":
		if ref && len(branch.Id) > 0 {
			fmt.Print("#" + branch.Id)
		} else {
			fmt.Print(branch.Id)
		}
	case "Name":
		fmt.Print(branch.Name)
	default:
		fmt.Print("")
	}
}
