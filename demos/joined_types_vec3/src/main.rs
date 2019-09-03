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
    name: String,
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

fn build_company() {

    let company_name = "Mycos";

    let mut company = Company::new(company_name);

    for i in 0..10 {
        println!("building shop {:?}", i);

        let s = Shop {name: "Mycos".to_string(), region: Regions::North };


    }
}

fn main() {
    println!("building shops...");
    build_company();
}
