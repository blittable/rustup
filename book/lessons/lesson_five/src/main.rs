use std::env;

// https://doc.rust-lang.org/rust-by-example/std_misc/arg/matching.html
fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            eprintln!("We need a command line argument!");
            return;
        }
        2 => {
            let cmd = &args[1];
            let i: i32 = match cmd.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("error in parsing the command line argument at position 1");
                    return;
                }
            };

            validate_age(i)
        }
        _ => {
            eprintln!("Error in parameters");
        }
    };
}

fn validate_age(age: i32) {
    println!("Checking Age");
    if age > 125 {
        panic! {
            println!("Age cannot be over 125 {:?} and your input age was: ", age)
        }
    }
}
pub mod vars;


use serde::{Serialize, Deserialize};
use std::fs;
use std::io;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use toml::from_str;
use std::io::ErrorKind;
use std::ffi::OsString;



#[derive(Debug, Deserialize)]
pub struct SuiteConfig {
    pub target_dir: Option<String>,
    pub file_size: Option<u64>,
    pub report_config: Option<ReportConfig>,
    pub tests: Option<Vec<TestConfig>>,
}

#[derive(Debug, Deserialize)]
pub struct ReportConfig {
    pub output_dir: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub test: String,
    pub target_dir: Option<String>,
    pub iterations: Option<u64>,
}

pub enum CoreTest { Async_Tokio_Random, Async_Tokio_Sequential, Tokio_Sync_Random, Tokio_Syc_Sequential } 

#[test]
fn read_config() {
    let toml_str = r#"
        target_dir = "/tmp/target"
        file_size_bytes = 5
        [report_config]
        output_dir = "/tmp/reports"
        [[tests]]
        test = "async"
        target_dir = "/tmp/target"
        iterations = 1
        [[tests]]
        test = "bsync"
        target_dir = "/tmp/target"
        iterations = 1
    "#;

    println!("test");
    let decoded: SuiteConfig = toml::from_str(toml_str).unwrap();
    println!("{:#?}", decoded);
}
use core::TestConfig;
use log::info;
use std::env;
use std::env::var_os;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time;

//use tracing::{instrument, Subscriber} ;
use tracing::{debug, instrument, Subscriber};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use tokio::fs;

use core::SuiteConfig;

const DEFAULT_CONFIG: &'static str = "/home/john/test_config.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("hello world!");

    configure_tracing();

    let args: Vec<String> = env::args().skip(1).collect();

    let config = build_config(args)?;

    println!("Config Values: {:?}", config.target_dir);

    let resp = run_harness(&config).await?;

    Ok(())
}

fn configure_tracing() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter("attrs_basic=trace")
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        let cat = 3;
        tracing::debug!("Debug output from trace");
    });
}

#[instrument]
pub fn build_config(args: Vec<String>) -> Result<SuiteConfig, Box<dyn std::error::Error>> {
    if args.len() > 0 {
        println!("Arg 1: {:?}", args[0]);

        let file = OpenOptions::new().open(&args[0]);

        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;
                let r: SuiteConfig = toml::from_str(&contents)?;

                return Ok(r);
            }
            Err(e) => {
                println!("Error opening the file: {:?}", e.description());
                return Err(Box::new(e));
            }
        };
    } else {
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .open(DEFAULT_CONFIG);

        match file {
            Ok(mut f) => {
                let mut contents = String::new();
                f.read_to_string(&mut contents)?;
                let r: SuiteConfig = toml::from_str(&contents)?;
                return Ok(r);
            }
            Err(e) => {
                println!("Error parsing the configuration file: '\n' description: {:?}, \n raw_os_error: {:?}, \n source: {:?}", e.description(), e.raw_os_error(), e.source());
                return Err(Box::new(e));
            }
        };
    }
}

///Simple sequential write
async fn run_write(config: &TestConfig, tag: i32, content: Vec<u8>) -> Result<(), Box<dyn Error>> {
    fs::write(
        "/tmp/load/foo.txt".to_string() + &tag.to_string(),
        content.as_slice(),
    )
    .await?;
    Ok(())
}

async fn run_harness(config: &SuiteConfig) -> Result<(), Box<dyn std::error::Error>> {
    let start = time::Instant::now();

    let mut payload = Vec::new();

    for t in config.tests.iter() {
        println!("Running Test: {:?}", t);
    }

    for _ in 0..10000 {
        payload.push(b'c');
    }

    // for i in 0..500 {
    //     let b = payload.clone();
    //     run_write(i, b).await?;
    // }

    // let duration = start.elapsed();
    // println!("Write time: {:?}", duration);

    // for i in 0..500 {
    //     let b = payload.clone();
    //     run_write(i, b).await?;
    // }

    let duration = start.elapsed();
    println!("F Write time: {:?}", duration);

    Ok(())
}
