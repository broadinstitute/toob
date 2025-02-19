use crate::cli::Args;
use crate::error::Error;
use std::fs::File;
use std::io::read_to_string;
use log::warn;

pub(crate) fn allele_reg2map(args: Args) -> Result<(), Error> {
    let file = args.get("f", "file", "file")?;
    for file in file {
        let json: serde_json::Value = serde_json::from_str(&read_to_string(File::open(file)?)?)?;
        let entries = json
            .as_array()
            .ok_or_else(|| Error::from("Input is not a JSON array"))?;
        for entry in entries {
            let entry = entry
                .as_object()
                .ok_or_else(|| Error::from("Entry is not a JSON object"))?;
            let id = entry
                .get("@id").and_then(|v| v.as_str()).and_then(|v| v.rsplit_once('/'));
            let id = if let Some((_, id)) = id {
                id
            } else {
                continue
            };
            let external_records = entry
                .get("externalRecords")
                .ok_or_else(|| Error::from("Entry has no externalRecords"))?
                .as_object()
                .ok_or_else(|| Error::from("Entry externalRecords is not a JSON object"))?;
            if let Some(gnomad2) = external_records.get("gnomAD_2") {
                let gnomad2 = gnomad2
                    .as_array()
                    .ok_or_else(|| Error::from("Entry gnomAD_2 is not an array"))?;
                for gnomad2_item in gnomad2 {
                    let gnomad2_item = gnomad2_item
                        .as_object()
                        .ok_or_else(|| Error::from("Entry gnomAD_2 is not a JSON object"))?;
                    let gnomad2_id = gnomad2_item
                        .get("id")
                        .ok_or_else(|| Error::from("Entry gnomAD_2 has no id"))?
                        .as_str()
                        .ok_or_else(|| Error::from("Entry gnomAD_2 id is not a string"))?;
                    println!("{}\t{}", gnomad2_id, id);
                }
            } else {
                warn!("No gnomAD_2 in entry '{}'", id);
            }
        }
    }
    Ok(())
}
