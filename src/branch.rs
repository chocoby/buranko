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
    pub fn parse(branch_name: &str) -> Self {
        if branch_name.is_empty() {
            return Self::empty();
        }

        // Case 1: feature/JRA-1234-foo
        if let Some(caps) = JIRA_FEATURE.captures(branch_name) {
            return Self {
                full_name: branch_name.to_string(),
                action: caps.get(1).map_or("", |m| m.as_str()).to_string(),
                id: caps
                    .get(2)
                    .and_then(|m| extract_jira_id(m.as_str()))
                    .unwrap_or_default(),
                description: caps.get(3).map_or("", |m| m.as_str()).to_string(),
            };
        }

        // Case 2: feature/1234_foo or feature/1234-foo or feature/1234 or feature/foo
        if let Some(caps) = FEATURE_BRANCH.captures(branch_name) {
            let action = caps.get(1).map_or("", |m| m.as_str()).to_string();
            if let Some(id) = caps.get(2) {
                // Case: feature/1234_foo or feature/1234-foo
                return Self {
                    full_name: branch_name.to_string(),
                    action,
                    id: id.as_str().to_string(),
                    description: caps.get(3).map_or("", |m| m.as_str()).to_string(),
                };
            } else if let Some(pure_id) = caps.get(4) {
                // Case: feature/1234
                return Self {
                    full_name: branch_name.to_string(),
                    action,
                    id: pure_id.as_str().to_string(),
                    description: String::new(),
                };
            } else {
                // Case: feature/foo
                return Self {
                    full_name: branch_name.to_string(),
                    action,
                    id: String::new(),
                    description: caps.get(5).map_or("", |m| m.as_str()).to_string(),
                };
            }
        }

        // Case 3: #1234-foo-bar
        if let Some(caps) = TICKET_REF.captures(branch_name) {
            return Self {
                full_name: branch_name.to_string(),
                action: String::new(),
                id: caps.get(1).map_or("", |m| m.as_str()).to_string(),
                description: caps.get(2).map_or("", |m| m.as_str()).to_string(),
            };
        }

        // Case 4: JRA-1234
        if let Some(caps) = JIRA_TICKET.captures(branch_name) {
            return Self {
                full_name: branch_name.to_string(),
                action: String::new(),
                id: caps.get(2).map_or("", |m| m.as_str()).to_string(),
                description: String::new(),
            };
        }

        // Case 5: Pure number
        if let Some(caps) = PURE_NUMBER.captures(branch_name) {
            return Self {
                full_name: branch_name.to_string(),
                action: String::new(),
                id: caps.get(1).map_or("", |m| m.as_str()).to_string(),
                description: String::new(),
            };
        }

        // Case 6: Simple text
        Self {
            full_name: branch_name.to_string(),
            action: String::new(),
            id: String::new(),
            description: branch_name.to_string(),
        }
    }

    fn empty() -> Self {
        Self {
            full_name: String::new(),
            action: String::new(),
            id: String::new(),
            description: String::new(),
        }
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
