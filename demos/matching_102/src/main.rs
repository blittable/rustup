use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn nice_read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    match read_username_from_file() {
        Ok(n) => {
            assert_eq!(n, "Jim");
            println!("Username was found in file");
        }
        Err(err) => println!("Error: {:?}", err),
    }

    match nice_read_username_from_file() {
        Ok(n) => {
            assert_eq!(n, "Jim\n");
            println!("Username was found in file");
        }
        Err(err) => println!("Error: {:?}", err),
    }
}
