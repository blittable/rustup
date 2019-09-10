enum Regions {
    North,
    South,
    East,
    West,
}

pub struct Company {
    name: String,
    branches: Vec<Shop>,
}

pub struct Shop {
    name: String,
    region: Regions,
}

impl Company {
    pub fn new(company_name: String) -> Self {
        Self {
            name: company_name,
            branches: Vec::new(),
        }
    }

    pub fn add_shop(&mut self, shop: Shop) {
        self.branches.push(shop);
    }
}

fn print_shops(company: &Company) {
    for s in company.branches.iter() {
        println!("We have shop: {:?}", &s.name);
    }
}

fn build_company() -> Company {
    let company_name = "Mycos";

    let mut company = Company::new(company_name.to_string());

    for i in 0..10 {
        println!("building shop {:?}", i);
        let s = Shop {
            name: ["Mycos ".to_string(), i.to_string()].concat(),
            region: Regions::North,
        };
        company.add_shop(s);
    }

    company
}

fn main() {
    println!("building shops...");
    let c = build_company();
    print_shops(&c);
}
