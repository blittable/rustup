use clap::{App, Arg, SubCommand};

fn main() {
    println!();
    let matches = App::new("Moobaan ERP")
        .version("1.0")
        .author("Theis uhp Rtotothip")
        .about("End-to-end Productivity Software for Property Development")
        .arg(Arg::with_name("server").help("Send a server command"))
        .subcommand(SubCommand::with_name("start").about("starts the Moobaan server"))
        .subcommand(SubCommand::with_name("stop").about("stops the Moobaan server"))
        .subcommand(SubCommand::with_name("status").about("gets the Moobaan server status"))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(o) = matches.value_of("server") {
        if let Some(matches) = matches.subcommand_matches("start") {
            println!("Starting Moobaan server...");
        }
        else if let Some(matches) = matches.subcommand_matches("stop") {
            println!("Stopping Moobaan server...");
        }
        else if let Some(matches) = matches.subcommand_matches("status") {
            println!("Getting Moobaan server status...");
        } else {
            println!("No parameter specified for the server...");
        }
    }

    inject_lines();
}

fn inject_lines() {
    println!();
    println!();
}
