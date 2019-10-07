extern crate serde;
extern crate toml;

#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use failure::Error;

#[derive(Debug, Fail)]
enum PerfOutputError {
    #[fail(display = "invalid toolchain name: {}", name)]
    InvalidPerfOutputName {
        name: String,
    },
    #[fail(display = "unknown toolchain version: {}", version)]
    UnknownPerfOutputVersion {
        version: String,
    }
}

pub struct PerfOutputId {
        id: i32, 
}

impl FromStr for PerfOutputId {
    type Err = PerfOutputError;

    fn from_str(s: &str) -> Result<PerfOutputId, PerfOutputError> {
        // ... etc
    }
}

pub type PerfOutputMap = HashMap<PerfOutputId, PathBuf>;

pub fn read_perf_outputfiles(path: PathBuf) -> Result<PerfOutputMap, Error>
{
    use std::fs::File;
    use std::io::Read;

    let mut string = String::new();
    File::open(path)?.read_to_string(&mut string)?;

    let toml: HashMap<String, PathBuf> = toml::from_str(&string)?;

    let toolchains = toml.iter().map(|(key, path)| {
        let toolchain_id = key.parse()?;
        Ok((toolchain_id, path))
    }).collect::<Result<PerfOutputMap, PerfOutputError>>()?;

    Ok(toolchains)
}