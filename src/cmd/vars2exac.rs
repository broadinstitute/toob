use crate::cli::Args;
use crate::error::Error;
use std::io::stdin;

pub(crate) fn vars2exac(args: Args) -> Result<(), Error> {
    let skip = args.check("s", "skip");
    let no_headers = args.check("nh", "no-headers");
    let mut lines = stdin().lines();
    if !no_headers {
        let line = lines.next().ok_or(Error::from("Input is empty"))??;
        println!("{}", line);
    }
    for line in lines {
        let line = line?;
        let mut parts = line.split([':', '/', '-', '_']);
        if let (Some(part1), Some(part2), Some(part3), Some(part4)) =
            (parts.next(), parts.next(), parts.next(), parts.next())
        {
            let trimmed_part1 = part1.trim_start_matches("chr");
            println!("{}-{}-{}-{}", trimmed_part1, part2, part3, part4);
        } else if !skip {
            return Err(Error::from(format!("Invalid variant id '{}", line)));
        }
    }
    Ok(())
}
