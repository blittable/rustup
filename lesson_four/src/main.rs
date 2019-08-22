//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'


fn get_full_name(fname: &str, lname: &str, mname: Option<&str>) -> String { // middle name can be empty
  match mname {
    Some(n) => format!("{} {} {}", fname, n, lname),
    None => format!("{} {}", fname, lname),
  }
}


fn get_divide_result(fNumber: f64, lNumber: f64) -> Result<f64, String>
{
    if lNumber == 0.0
    {
      Err("Cannot use 0 to divide".to_string())
    }    
    else
    {
        Ok(fNumber/lNumber)
    }
}

fn main() {
   println!("{}", get_full_name("Galileo", "Galilei", None));
   println!("{}", get_full_name("Leonardo", "Vinci", Some("Da")));

   match get_divide_result(10.0, 0.0) {
        Err(why) => println!("{}", why),
        Ok(result) => println!("{}",result),
    }

}
