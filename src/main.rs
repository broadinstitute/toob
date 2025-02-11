use toob::cli::Args;
use toob::cmd::Cmds;
use toob::error::Error;

fn main() -> Result<(), Error>{
    let args = Args::new();
    let cmds = Cmds::new();
    let cmd_str = args.get_cmd().ok_or("No command provided")?;
    cmds.get(cmd_str).ok_or(Error::from(format!("Unknown command '{}'", cmd_str)))?(args)?;
    Ok(())
}