use std::collections::BTreeMap;
use std::env;
use std::fmt::{Display, Formatter};
use crate::error::Error;

pub struct Args {
    args: BTreeMap<String, Vec<String>>,
}

impl Args {
    pub(crate) fn new() -> Args {
        let mut args = BTreeMap::new();
        let mut key: String = "".to_string();
        for arg in env::args() {
            if arg.starts_with('-') {
                key = arg.trim_start_matches('-').to_string();
                args.entry(key.clone()).or_insert(vec![]);
            } else {
                args.entry(key.clone()).or_insert(vec![]).push(arg);
            }
        }
        Args { args}
    }
    pub(crate) fn get_cmd(&self) -> Option<&String> {
        self.args.get("").and_then(|v| v.get(1))
    }
    pub(crate) fn get(&self, key1: &str, key2: &str, name: &str) -> Result<&Vec<String>, Error> {
        self.args.get(key1).or(self.args.get(key2))
            .ok_or(Error::from(
                format!("Missing argument for {}. ('-{}' or '-{}')", name, key1, key2)
            ))
    }
    pub(crate) fn check(&self, key1: &str, key2: &str) -> bool {
        self.args.contains_key(key1) || self.args.contains_key(key2)
    }
}

impl Default for Args {
    fn default() -> Self {
        Args::new()
    }
}

impl Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (key, values) in &self.args {
            writeln!(f, "{} -> {}", key, values.join(", "))?;
        }
        Ok(())
    }
}