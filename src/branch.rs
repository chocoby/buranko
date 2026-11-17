#[derive(Debug, PartialEq)]
pub struct Branch {
    pub full_name: String,
    pub action: String,
    pub id: String,
    pub description: String,
}

use regex::Regex;
use std::sync::LazyLock;

static FEATURE_BRANCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([\w-]+)/(?:(\d+)[_-](.+)|(\d+)$|(.+))$").unwrap());

static JIRA_FEATURE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([\w-]+)/([A-Z]+-\d+)-(.+)$").unwrap());

static TICKET_REF: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^#?(\d+)-(.+)$").unwrap());

static JIRA_TICKET: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^([A-Z]+-(\d+))$").unwrap());

static PURE_NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)$").unwrap());

impl Branch {
    fn new(full_name: &str, action: &str, id: &str, description: &str) -> Self {
        Self {
            full_name: full_name.to_string(),
            action: action.to_string(),
            id: id.to_string(),
            description: description.to_string(),
        }
    }

    pub fn parse(branch_name: &str) -> Self {
        // Match specific JIRA branches before the general action/name pattern.
        if let Some(caps) = JIRA_FEATURE.captures(branch_name) {
            let id = extract_jira_id(&caps[2]).unwrap_or_default();
            return Self::new(branch_name, &caps[1], &id, &caps[3]);
        }

        if let Some(caps) = FEATURE_BRANCH.captures(branch_name) {
            let id = caps
                .get(2)
                .or_else(|| caps.get(4))
                .map_or("", |m| m.as_str());
            let description = caps
                .get(3)
                .or_else(|| caps.get(5))
                .map_or("", |m| m.as_str());
            return Self::new(branch_name, &caps[1], id, description);
        }

        if let Some(caps) = TICKET_REF.captures(branch_name) {
            return Self::new(branch_name, "", &caps[1], &caps[2]);
        }

        if let Some(caps) = JIRA_TICKET.captures(branch_name) {
            return Self::new(branch_name, "", &caps[2], "");
        }

        if let Some(caps) = PURE_NUMBER.captures(branch_name) {
            return Self::new(branch_name, "", &caps[1], "");
        }

        Self::new(branch_name, "", "", branch_name)
    }
}

// Helper function to extract JIRA-style IDs (e.g., JRA-1234)
fn extract_jira_id(text: &str) -> Option<String> {
    let parts: Vec<&str> = text.split('-').collect();
    if parts.len() >= 2 && parts[1].chars().all(|c| c.is_ascii_digit()) {
        Some(parts[1].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_branch_name() {
        let test_cases = vec![
            (
                "feature/1234_foo",
                Branch {
                    full_name: "feature/1234_foo".to_string(),
                    action: "feature".to_string(),
                    id: "1234".to_string(),
                    description: "foo".to_string(),
                },
            ),
            (
                "feature/1234_foo-bar",
                Branch {
                    full_name: "feature/1234_foo-bar".to_string(),
                    action: "feature".to_string(),
                    id: "1234".to_string(),
                    description: "foo-bar".to_string(),
                },
            ),
            (
                "feature/1234-foo-bar",
                Branch {
                    full_name: "feature/1234-foo-bar".to_string(),
                    action: "feature".to_string(),
                    id: "1234".to_string(),
                    description: "foo-bar".to_string(),
                },
            ),
            (
                "feature/1234",
                Branch {
                    full_name: "feature/1234".to_string(),
                    action: "feature".to_string(),
                    id: "1234".to_string(),
                    description: "".to_string(),
                },
            ),
            (
                "feature/foo",
                Branch {
                    full_name: "feature/foo".to_string(),
                    action: "feature".to_string(),
                    id: "".to_string(),
                    description: "foo".to_string(),
                },
            ),
            (
                "#1234-foo-bar",
                Branch {
                    full_name: "#1234-foo-bar".to_string(),
                    action: "".to_string(),
                    id: "1234".to_string(),
                    description: "foo-bar".to_string(),
                },
            ),
            (
                "feature/JRA-1234-foo",
                Branch {
                    full_name: "feature/JRA-1234-foo".to_string(),
                    action: "feature".to_string(),
                    id: "1234".to_string(),
                    description: "foo".to_string(),
                },
            ),
            (
                "JRA-1234",
                Branch {
                    full_name: "JRA-1234".to_string(),
                    action: "".to_string(),
                    id: "1234".to_string(),
                    description: "".to_string(),
                },
            ),
            (
                "foo",
                Branch {
                    full_name: "foo".to_string(),
                    action: "".to_string(),
                    id: "".to_string(),
                    description: "foo".to_string(),
                },
            ),
            (
                "",
                Branch {
                    full_name: "".to_string(),
                    action: "".to_string(),
                    id: "".to_string(),
                    description: "".to_string(),
                },
            ),
        ];

        for (input, expected) in test_cases {
            assert_eq!(Branch::parse(input), expected);
        }
    }
}
