mod susudb;

extern crate clap;

use clap::{App, SubCommand};
use crate::susudb::traits::SusuDataTraits;
use crate::susudb::traits::SusuDatabaseTraits;
use crate::susudb::SusuData;
use crate::susudb::SusuDatabase;

fn main() {
    println!("*** Hello, susu db! ***");

    let matches = get_matcher();
    if let Some(matches) = matches.subcommand_matches("add") {
        add_element(matches);
    }
    else if let Some(matches) = matches.subcommand_matches("get") {
        get_value(matches);
    }
    else {
        println!("Please enter command.");
    }

    let mut database = SusuDatabase::new();
    database.config("susu_db");

    // Test add data
    for key_temp in 0..100 {
        let new_data =
            SusuData::new_data(&format!("key#{}", key_temp), &format!("rust_{}", key_temp));
        database.add(new_data);
    }

    // Test update data
    for key_temp in 0..100 {
        let new_data = SusuData::new_data(
            &format!("key#{}", key_temp),
            &format!("rust_updated_v1_{}", key_temp),
        );
        database.add(new_data);
    }

    // Test get data
    for key_temp in 0..102 {
        let value = database.get(&format!("key#{}", key_temp));
        match value {
            Some(val) => println!("Found value: {:?}\n", val),
            None => println!("Not found any data\n"),
        }
    }

    // Test add invalid data
    let new_data = SusuData::new_data(&"", &"rust_empty");
    database.add(new_data);
}

fn get_matcher() -> clap::ArgMatches<'static> {
    App::new("susu")
                        .version("0.1.0")
                        .author("Mycos RUST class <Mycostech.com>")
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

fn add_element(matches :&clap::ArgMatches<'static>) {
    let key = matches.value_of("KEY").unwrap();
    let value = matches.value_of("VALUE").unwrap();
    println!("{} {}", key, value);
}

fn get_value(matches :&clap::ArgMatches<'static>) {
    let key = matches.value_of("KEY").unwrap();
    println!("{}", key);
}