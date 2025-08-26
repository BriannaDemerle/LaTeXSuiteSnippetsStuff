use json;
use regex::Regex;
use std::{
    error::Error,
    fmt::{Debug, Display},
    fs::read_to_string,
};

pub enum SnippyError {}

impl Display for SnippyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "Snippy Error..."),
        }
    }
}

pub enum SnippetsError {
    IOError(std::io::Error),
    JsonError(json::Error),
    BadJsonFormat,
    SnippyError(SnippyError),
}

impl From<std::io::Error> for SnippetsError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

impl From<json::Error> for SnippetsError {
    fn from(value: json::Error) -> Self {
        Self::JsonError(value)
    }
}

impl From<SnippyError> for SnippetsError {
    fn from(value: SnippyError) -> Self {
        SnippetsError::SnippyError(value)
    }
}

impl Debug for SnippetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadJsonFormat => write!(f, "Could not find the snippets..."),
            Self::IOError(e) => write!(f, "IO Error: {e}"),
            Self::JsonError(e) => write!(f, "Json Error: {e}"),
            Self::SnippyError(e) => write!(f, "Snippy Error: {e}"),
        }
    }
}

impl Display for SnippetsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Error for SnippetsError {}

pub fn save_snippets<P: AsRef<std::path::Path>>(path: P) -> Result<(), SnippetsError> {
    let s = read_to_string(path)?;

    let trigger_re = Regex::new(r#"trigger: \"(.+?)\""#).unwrap();
    let replace_re = Regex::new(r#"replacement: \"(.+?)\""#).unwrap();
    let options_re = Regex::new(r#"options: \"(.+?)\""#).unwrap();

    let snippets = json::parse(&s)?
        .entries_mut()
        .find(|(s, _)| *s == "snippets")
        .ok_or(SnippetsError::BadJsonFormat)?
        .1
        .take_string()
        .ok_or(SnippetsError::BadJsonFormat)?
        .replace(r"\\", "\\")
        .lines()
        .map(|s| s.trim().to_string())
        .filter_map(|s| {
            Some((
                options_re
                    .captures(&s)
                    .map(|c| c.extract::<1>().1[0])?
                    .to_string(),
                trigger_re
                    .captures(&s)
                    .map(|c| c.extract::<1>().1[0])?
                    .to_string(),
                replace_re
                    .captures(&s)
                    .map(|c| c.extract::<1>().1[0])?
                    .to_string(),
            ))
        })
        .map(|(options, trigger, replace)| format!("{trigger} := {replace} ! {options}\n"))
        .collect::<String>();

    std::fs::write("./snippets.txt", snippets)?;

    Ok(())
}

pub fn export_snippets<P: AsRef<std::path::Path>>(path: P) -> Result<(), SnippetsError> {
    let snippets = std::fs::read_to_string("./snippets.txt")?
        .lines()
        .filter_map(|s| s.split_once(":="))
        .filter_map(|(trigger, rest)| {
            let (replace, options) = rest.split_once('!')?;
            Some((
                trigger.trim().to_string(),
                replace.trim().to_string(),
                options.trim().to_string(),
            ))
        })
        .filter_map(|(t, r, o)| {
            Some(json::object! {
                trigger: t,
                replacement: r,
                options: o
            })
        })
        .collect::<Vec<_>>();

    let s = std::fs::read_to_string(&path)?;
    let mut data = json::parse(&s)?;
    data.remove("snippets");
    let snippets = json::from(snippets);
    data.insert("snippets", snippets.dump())?;

    std::fs::write(path, json::stringify_pretty(data, 4))?;

    Ok(())
}

fn main() -> Result<(), SnippetsError> {
    export_snippets("data.json")?;
    Ok(())
}
