use std::io::stdin;
use serde_json::Value;
use crate::cli::Args;
use crate::error::Error;

pub(crate) fn json2tsv(args: Args) -> Result<(), Error> {
    let headers = args.get("f", "fields", "selected fields")?;
    let no_headers = args.check("nh", "no-headers");
    if !no_headers {
        println!("{}", headers.join("\t"));
    }
    for line in stdin().lines() {
        let line = line?;
        let value: Value = serde_json::from_str(&line)?;
        match value {
            Value::Object(map) => {
                let mut header_iter = headers.iter();
                if let Some(header) = header_iter.next() {
                    print_field(header, &map);
                    for header in header_iter {
                        print!("\t");
                        print_field(header, &map);
                    }
                }
            }
            _ => {
                return Err(Error::from("Input is not a JSON object"));
            }
        }
        println!();
    }
    Ok(())
}

fn print_field(header: &str, map: &serde_json::Map<String, Value>) {
    if let Some(value) = map.get(header) {
        if let Value::String(s) = value {
            print!("{}", s);
        } else {
            print!("{}", value);
        }
    }
}