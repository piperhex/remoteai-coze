use remoteai_coze::{
    bootstrap::{self, Bootstrap},
    error::{Error, Result},
    mcp,
};
use std::{io::BufRead, path::PathBuf};

const MAX_BOOTSTRAP_BYTES: u64 = 8192;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut arguments = std::env::args().skip(1);
    let argument = arguments.next().ok_or(Error::Invalid)?;
    if let Some(path) = argument.strip_prefix("--coze-mcp=") {
        return mcp::run_stdio(&PathBuf::from(path));
    }
    match argument.as_str() {
        "--version" => {
            println!("{}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
        "--serve" if arguments.next().is_none() => {
            let mut line = String::new();
            std::io::Read::take(std::io::stdin().lock(), MAX_BOOTSTRAP_BYTES + 1)
                .read_line(&mut line)?;
            bootstrap::serve(parse(&line)?, true)
        }
        "--config" => {
            let path = arguments.next().ok_or(Error::Invalid)?;
            if arguments.next().is_some() {
                return Err(Error::Invalid);
            }
            let mut text = String::new();
            std::io::Read::read_to_string(
                &mut std::io::Read::take(std::fs::File::open(path)?, MAX_BOOTSTRAP_BYTES + 1),
                &mut text,
            )?;
            bootstrap::serve(parse(&text)?, false)
        }
        _ => Err(Error::Invalid),
    }
}

fn parse(text: &str) -> Result<Bootstrap> {
    if text.len() as u64 > MAX_BOOTSTRAP_BYTES {
        return Err(Error::Invalid);
    }
    serde_json::from_str(text).map_err(|_| Error::Invalid)
}
