use log::LevelFilter;
use simplelog::{ColorChoice, Config as LogConfig, TermLogger, TerminalMode};
use crate::cli::Args;
use crate::cmd::Cmds;
use crate::error::Error;

pub mod cli;
pub mod cmd;
pub mod error;

pub fn run_cli() -> Result<(), Error> {
    TermLogger::init(
        LevelFilter::Info,
        LogConfig::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto
    ).unwrap();
    let args = Args::new();
    let cmds = Cmds::new();
    let cmd_str = args.get_cmd().ok_or(
        format!("No command provided. {}", cmds.known_cmds_are()))?;
    cmds.get(cmd_str)
        .ok_or(Error::from(
            format!("Unknown command '{}'. {}", cmd_str, cmds.known_cmds_are())))?(args)?;
    Ok(())
}

