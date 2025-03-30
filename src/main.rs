use clap::Parser;
use git2::Repository;
use std::io::{self, Read};

mod branch;
use branch::Branch;

/// CLI tool to parse and format git branch names
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Format output using template
    #[arg(short, long)]
    template: bool,

    /// Select output field (FullName, Action, ID, LinkID, Description)
    #[arg(long, value_parser = ["FullName", "Action", "ID", "LinkID", "Description"], default_value = "ID")]
    output: String,

    /// Show all branch information
    #[arg(short, long)]
    verbose: bool,
}

fn read_from_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn get_current_branch() -> Result<Branch, git2::Error> {
    let repo = Repository::discover(".")?;
    let head = repo.head()?;
    let branch_name = head.shorthand().unwrap_or("HEAD detached").to_string();
    Ok(Branch::parse(&branch_name))
}

fn get_template_from_config() -> Result<String, git2::Error> {
    let repo = Repository::discover(".")?;
    let config = repo.config()?;
    let template = config.get_string("buranko.template").unwrap_or_default();
    Ok(template)
}

fn format_with_template(branch: &Branch, template: &str) -> String {
    let mut result = template.to_string();
    result = result.replace("{FullName}", &branch.full_name);
    result = result.replace("{Action}", &branch.action);
    result = result.replace("{ID}", &branch.id);
    result = result.replace("{LinkID}", &format!("#{}", &branch.id));
    result = result.replace("{Description}", &branch.description);
    result
}

fn format_verbose_output(branch: &Branch) -> String {
    format!(
        "Full Name: {}\nAction: {}\nID: {}\nDescription: {}",
        branch.full_name, branch.action, branch.id, branch.description
    )
}

fn main() {
    let args = Args::parse();

    // Check if input is coming from a pipe
    let branch = if !atty::is(atty::Stream::Stdin) {
        // Read from stdin
        match read_from_stdin() {
            Ok(input) => Branch::parse(&input),
            Err(e) => {
                eprintln!("Failed to read from stdin: {}", e);
                return;
            }
        }
    } else {
        // Get current git branch
        match get_current_branch() {
            Ok(branch) => branch,
            Err(e) => {
                eprintln!("Failed to get branch name: {}", e);
                return;
            }
        }
    };

    if args.verbose {
        println!("{}", format_verbose_output(&branch));
    } else if args.template {
        match get_template_from_config() {
            Ok(template) => {
                if !template.is_empty() {
                    let formatted = format_with_template(&branch, &template);
                    print!("{}", formatted);
                } else {
                    println!(
                        "No template configured. Use 'git config buranko.template' to set a template."
                    );
                }
            }
            Err(e) => eprintln!("Failed to get template: {}", e),
        }
    } else {
        let output = match args.output.as_str() {
            "FullName" => branch.full_name,
            "Action" => branch.action,
            "ID" => branch.id,
            "LinkID" => format!("#{}", branch.id),
            "Description" => branch.description,
            _ => unreachable!("Invalid output field - clap should prevent this"),
        };
        print!("{}", output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_output_field() {
        let test_branch = Branch {
            full_name: "feature/123-test".to_string(),
            action: "feature".to_string(),
            id: "123".to_string(),
            description: "test".to_string(),
        };

        // Test each output field
        let cases = vec![
            ("FullName", "feature/123-test"),
            ("Action", "feature"),
            ("ID", "123"),
            ("LinkID", "#123"),
            ("Description", "test"),
        ];

        for (field, expected) in cases {
            let output = match field {
                "FullName" => test_branch.full_name.clone(),
                "Action" => test_branch.action.clone(),
                "ID" => test_branch.id.clone(),
                "LinkID" => format!("#{}", test_branch.id),
                "Description" => test_branch.description.clone(),
                _ => panic!("Invalid test case"),
            };
            assert_eq!(output, expected);
        }
    }

    #[test]
    fn test_default_output_field() {
        use clap::Parser;

        // Parse empty args (no --output specified)
        let args = Args::try_parse_from(["buranko"]).unwrap();

        // Check that default value is "ID"
        assert_eq!(args.output, "ID");
    }

    #[test]
    fn test_verbose_output() {
        let test_branch = Branch {
            full_name: "feature/123-test".to_string(),
            action: "feature".to_string(),
            id: "123".to_string(),
            description: "test".to_string(),
        };

        let expected = "Full Name: feature/123-test\n\
                       Action: feature\n\
                       ID: 123\n\
                       Description: test";

        assert_eq!(format_verbose_output(&test_branch), expected);
    }
}
