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
