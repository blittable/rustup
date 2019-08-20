//Basics of Traits - Homework

struct Person {
    name: String
}

trait Funny {
    fn make_fun(&self) -> String;
}

impl Funny for Person {
    fn make_fun(&self) -> String {
        return self.name.chars().rev().collect();
    }
}

fn main() {
    let boss = Person {
        name: "nimda".to_string()
    };
    let haruka = Person {
        name: "春香".to_string()
    };

    let new_name = boss.make_fun();
    println!("new boss name = {}", new_name);

    let new_name = haruka.make_fun();
    println!("new haruka name = {}", new_name);

    let anony = Person::make_fun(&Person { name: "abc".to_string() });
    println!("{}", anony)
}
