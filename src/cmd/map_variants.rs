use crate::cli::Args;
use crate::error::Error;

pub(crate) fn map_variants(args: Args) -> Result<(), Error> {
    const URL: &str = "http://reg.clinicalgenome.org//alleles?file=ExAC.id";
    let tokio =
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .enable_all()
            .build()?;
    let client = reqwest::Client::new();
    let response = tokio.block_on(client.get(URL).send())?;
    println!("{}", args);
    todo!()
}