#[allow(dead_code)]

struct App<'a> {
    data: Vec<(&'a str, u64)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            data: vec![("Khem", 106), ("Neng", 112), ("Kannon", 106)],
        }
    }

    fn increment_last_value(&mut self, new_value: u64) {
        match self.data.pop() {
            Some(i) => {
                let i = (i.0, i.1 + new_value);
                self.data.push(i);
            }
            None => println!("Attempted to update the value of an empty pair"),
        }
    }

    fn for_each_implicit(&mut self, new_value: u64) {
        for pair in self.data.iter().map(|letter| letter).filter(|y| y.1 > 106) {
            println!("{:?}", pair);
        }
    }

    fn print(&mut self) {
        for pair in self.data.iter().map(|letter| letter).filter(|y| y.1 > 106) {
            println!("{:?}", pair);
        }
    }
}

fn main() {
    let mut app = App::new();
    app.print();
    app.increment_last_value(2);
    app.print();
    app.increment_last_value(2);
}
