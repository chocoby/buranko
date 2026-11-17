use clap::Parser;
use git2::Repository;
use std::io::{self, IsTerminal, Read};

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
    static PLACEHOLDER: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"\{(FullName|Action|ID|LinkID|Description)\}").unwrap()
    });
    PLACEHOLDER
        .replace_all(template, |caps: &regex::Captures<'_>| match &caps[1] {
            "FullName" => branch.full_name.clone(),
            "Action" => branch.action.clone(),
            "ID" => branch.id.clone(),
            "LinkID" => format!("#{}", branch.id),
            "Description" => branch.description.clone(),
            _ => unreachable!(),
        })
        .into_owned()
}

fn format_verbose_output(branch: &Branch) -> String {
    format!(
        "Full Name: {}\nAction: {}\nID: {}\nDescription: {}",
        branch.full_name, branch.action, branch.id, branch.description
    )
}

fn main() {
    let args = Args::parse();

    if let Err(error) = run(&args) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), Box<dyn std::error::Error>> {
    let input = if io::stdin().is_terminal() {
        String::new()
    } else {
        read_from_stdin().map_err(|e| format!("Failed to read from stdin: {e}"))?
    };
    let branch = if input.is_empty() {
        get_current_branch().map_err(|e| format!("Failed to get branch name: {e}"))?
    } else {
        Branch::parse(&input)
    };

    if args.verbose {
        println!("{}", format_verbose_output(&branch));
    } else if args.template {
        let template =
            get_template_from_config().map_err(|e| format!("Failed to get template: {e}"))?;
        if template.is_empty() {
            return Err(
                "No template configured. Use 'git config buranko.template' to set a template."
                    .into(),
            );
        }
        print!("{}", format_with_template(&branch, &template));
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
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
