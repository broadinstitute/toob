use crate::cli::Args;
use crate::error::Error;
use std::fs::File;
use std::io::read_to_string;

pub(crate) fn parse_allele_reg(args: Args) -> Result<(), Error> {
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
            let genomic_alleles = entry
                .get("genomicAlleles")
                .ok_or_else(|| Error::from("Entry has no genomicAlleles"))?
                .as_array()
                .ok_or_else(|| Error::from("Entry genomicAlleles is not an array"))?;
            for genomic_allele in genomic_alleles {
                let genomic_allele = genomic_allele
                    .as_object()
                    .ok_or_else(|| Error::from("Entry genomicAllele is not a JSON object"))?;
                let reference_genome = genomic_allele
                    .get("referenceGenome").and_then(|v| v.as_str());
                if reference_genome == Some("GRCh37") {
                    let chromosome = genomic_allele
                        .get("chromosome")
                        .ok_or_else(|| Error::from("Entry genomicAllele has no chromosome"))?
                        .as_str()
                        .ok_or_else(|| {
                            Error::from("Entry genomicAllele chromosome is not a string")
                        })?;
                    let coordinates_list = genomic_allele
                        .get("coordinates")
                        .ok_or_else(|| Error::from("Entry genomicAllele has no coordinates"))?
                        .as_array()
                        .ok_or_else(|| {
                            Error::from("Entry genomicAllele coordinates is not an array")
                        })?;
                    for coordinates in coordinates_list {
                        let coordinates = coordinates.as_object().ok_or_else(|| {
                            Error::from("Entry genomicAllele coordinates is not a JSON object")
                        })?;
                        let start = coordinates
                            .get("start")
                            .ok_or_else(|| {
                                Error::from("Entry genomicAllele coordinates has no start")
                            })?
                            .as_u64()
                            .ok_or_else(|| {
                                Error::from("Entry genomicAllele coordinates start is not a number")
                            })?;
                        let reference_allele =
                            coordinates.get("referenceAllele")
                                .ok_or_else(|| Error::from("Entry genomicAllele coordinates has no referenceAllele"))?
                                .as_str()
                                .ok_or_else(|| Error::from("Entry genomicAllele coordinates referenceAllele is not a string"))?;
                        let allele = coordinates
                            .get("allele")
                            .ok_or_else(|| {
                                Error::from("Entry genomicAllele coordinates has no allele")
                            })?
                            .as_str()
                            .ok_or_else(|| {
                                Error::from(
                                    "Entry genomicAllele coordinates allele is not a string",
                                )
                            })?;
                        println!(
                            "{}:{}:{}:{}\t{}",
                            chromosome, start, reference_allele, allele, id
                        );
                    }
                }
            }
        }
    }
    Ok(())
}
