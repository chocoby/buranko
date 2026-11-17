use std::borrow::Cow;

use clap::ValueEnum;

use crate::branch::Branch;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
#[value(rename_all = "verbatim")]
pub enum OutputField {
    FullName,
    Action,
    ID,
    LinkID,
    Description,
}

impl OutputField {
    pub fn value(self, branch: &Branch) -> Cow<'_, str> {
        match self {
            Self::FullName => Cow::Borrowed(&branch.full_name),
            Self::Action => Cow::Borrowed(&branch.action),
            Self::ID => Cow::Borrowed(&branch.id),
            Self::LinkID => Cow::Owned(format!("#{}", branch.id)),
            Self::Description => Cow::Borrowed(&branch.description),
        }
    }
}

pub fn format_with_template(branch: &Branch, template: &str) -> String {
    static PLACEHOLDER: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        let names = OutputField::value_variants()
            .iter()
            .map(|field| field.to_possible_value().unwrap().get_name().to_owned())
            .collect::<Vec<_>>()
            .join("|");
        regex::Regex::new(&format!(r"\{{({names})\}}")).unwrap()
    });
    PLACEHOLDER
        .replace_all(template, |caps: &regex::Captures<'_>| {
            OutputField::from_str(&caps[1], false)
                .expect("placeholder matches a known output field")
                .value(branch)
        })
        .into_owned()
}

pub fn format_verbose_output(branch: &Branch) -> String {
    format!(
        "Full Name: {}\nAction: {}\nID: {}\nDescription: {}",
        branch.full_name, branch.action, branch.id, branch.description
    )
}
