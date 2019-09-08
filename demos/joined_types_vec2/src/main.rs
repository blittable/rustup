enum Regions {
    North,
    South,
    East,
    West,
}

pub struct Company<'a> {
    name: &'a str,
    branches: Vec<Shop>
}

pub struct Shop {
    name: &'static str,
    region: Regions,
}

impl<'a> Company<'a> {
    pub fn new(company_name: &'a str) -> Self {
        Self {
            name: company_name,
            branches: Vec::new(),
        }
    }

    pub fn add_shop(&mut self, shop: Shop) {
        self.branches.push(shop);
    }
}

fn build_company<'a>() -> Company<'a> {

    let company_name = "Mycos";

    let mut company = Company::new(company_name);

    for i in 0..10 {
        println!("building shop {:?}", i);
        let t = format!("{}{}", "mycos", i);
        let s = Shop {name: t.to_owned().as_str(), region: Regions::North };
        company.add_shop(s);
    }

    company
}

fn print_shops(company: &Company) {
    for s in company.branches.iter() {
        println!("We have shop: {:?}", &s.name);
    } 
}

fn main() {
    println!("building shops...");
    let c = build_company();
    print_shops(&c); 
}
