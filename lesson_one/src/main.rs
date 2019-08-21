// ============================
// Setup and returning a string
// ============================

static MYCOS: &str = "Mycos ";
static COMPANY: &str = "Company ";
static FIRSTAPP: &str = "First appeared";
static CYEAR: i32 = 2008;

fn main() {
    //#1
    println! ("{0}:{1}",FIRSTAPP,first_appeared());

    //#2
    let c1 = String::from(MYCOS);
    let company = get_company(c1);
    prin_message(company);
}

//#1 
fn first_appeared() -> i32 { CYEAR } 

//#2
fn get_company(mut company: String) -> String
{
      company.push_str(COMPANY);
      return company;
}

//#Other
fn prin_message(value:String){
    println!("{}",value);
}
