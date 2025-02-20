use crate::cli::Args;
use crate::error::Error;
use crate::util::json::JsonCursor;
use log::warn;
use serde_json::Value;
use std::fs::File;
use std::io::read_to_string;

pub(crate) fn allele_reg2map(args: Args) -> Result<(), Error> {
    let file = args.get("f", "file", "file")?;
    for file in file {
        let json: Value = serde_json::from_str(&read_to_string(File::open(file)?)?)?;
        for entry in JsonCursor::new(&json).as_array()?.iter() {
            let entry = entry.as_object()?;
            let id =
                entry.get("@id")?.as_str_opt().and_then(|v| v.rsplit_once('/'));
            let id = if let Some((_, id)) = id { id } else { continue };
            let external_records_cursor = entry.get("externalRecords")?;
            let external_records = external_records_cursor.as_object()?;
            if let Some(gnomad2) = external_records.get_opt("gnomAD_2") {
                for gnomad2_item in gnomad2.as_array()?.iter() {
                    let gnomad2_item = gnomad2_item.as_object()?;
                    let gnomad2_id_cursor = gnomad2_item.get("id")?;
                    let gnomad2_id = gnomad2_id_cursor.as_str()?;
                    println!("{}\t{}", gnomad2_id, id);
                }
            } else {
                warn!("No gnomAD_2 in entry '{}'", id);
            }
        }
    }
    Ok(())
}
