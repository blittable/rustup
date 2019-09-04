mod susudb;

extern crate clap;

use clap::{App, SubCommand};
use crate::susudb::traits::SusuDataTraits;
use crate::susudb::traits::SusuDatabaseTraits;
use crate::susudb::SusuData;
use crate::susudb::SusuDatabase;

fn main() {
    println!("*** 🤷‍  Hello, susu db! 🤦 ***");

    let matches = get_matcher();
    let mut database = SusuDatabase::new();
    database.config("susu_db");
    
    if let Some(matches) = matches.subcommand_matches("add") {
        add_element(database,matches);
    }
    else if let Some(matches) = matches.subcommand_matches("get") {
        get_value(database,matches);
    }
    else {
        println!("Please enter command.");
    }

   
}

fn get_matcher() -> clap::ArgMatches<'static> {
    App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about("SUSU clap SUSU clap clap")
                        .subcommand(SubCommand::with_name("add")
                                    .about("add element <KEY> <VALUE> into DB (EX: add 1 'First Last')")
                                    .arg_from_usage("<KEY> 'Key of element'")
                                    .arg_from_usage("<VALUE> 'Values of element'"))
                        .subcommand(SubCommand::with_name("get")
                                    .about("get element value by <KEY> (EX: get 1)")
                                    .arg_from_usage("<KEY> 'Key of element'"))
                        .get_matches()
}

fn add_element(database:SusuDatabase, matches :&clap::ArgMatches<'static>) {
    let key = matches.value_of("KEY").unwrap();
    let value = matches.value_of("VALUE").unwrap();

    let result = database.add(SusuData{key:key.to_string(),value:Some(value.to_string())});
    if result == true
    {
        println!("> success!");
    }
    else
    {
        println!("> failed!");
    }
    
}

fn get_value(database:SusuDatabase,matches :&clap::ArgMatches<'static>) {
    let key = matches.value_of("KEY").unwrap();
    match database.get(key)
    {
        Some(value) => println!("> The value is: {:?}", value),
        None => println!("> No return value")
    }
    
}