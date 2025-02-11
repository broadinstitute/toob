use std::collections::BTreeMap;
use std::env;
use std::fmt::{Display, Formatter};

pub struct Args {
    args: BTreeMap<String, Vec<String>>,
}

impl Args {
    pub fn new() -> Args {
        let mut args = BTreeMap::new();
        let mut key: String = "".to_string();
        for arg in env::args() {
            if arg.starts_with('-') {
                key = arg.trim_start_matches('-').to_string();
            } else {
                args.entry(key.clone()).or_insert(vec![]).push(arg);
            }
        }
        Args { args}
    }
    pub fn get_cmd(&self) -> Option<&String> {
        self.args.get("").and_then(|v| v.get(1))
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