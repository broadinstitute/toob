use std::io::stdin;
use serde_json::Value;
use crate::cli::Args;
use crate::error::Error;

pub(crate) fn json2tsv(args: Args) -> Result<(), Error> {
    println!("{}", args);
    for line in stdin().lines() {
        let line = line?;
        let value: Value = serde_json::from_str(&line)?;
        println!("{}", value);
    }
    todo!()
}