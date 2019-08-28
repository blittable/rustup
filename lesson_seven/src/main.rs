mod material;
mod liquid;
mod energy;
use liquid::Property;

fn main() {
    let mut mats = vec![
        material::of::Ingrediant {
            name: "Coffee".to_string()
        }
    ];
    let mut liqs = vec![
        liquid::is::Mixer {
            name: "Water".to_string(),
            temp: energy::form::Temperature {
                value: 15,
                unit: "celsius".to_string()
            }
        }
    ];

    let cup_a = brew(&mats, &liqs);
    println!("Cup A is : {}", cup_a);

    mats = vec![
        material::of::Ingrediant {
            name: "Coffee".to_string()
        },
        material::of::Ingrediant {
            name: "Sugar".to_string()
        }
    ];
    liqs = vec![
        liquid::is::Mixer {
            name: "Water".to_string(),
            temp: energy::form::Temperature {
                value: 30,
                unit: "celsius".to_string()
            }
        },
        liquid::is::Mixer {
            name: "Water".to_string(),
            temp: energy::form::Temperature {
                value: 0,
                unit: "celsius".to_string()
            }
        }];

    let cup_b = brew(&mats, &liqs);
    println!("Cup B is : {}", cup_b);

    mats = vec![
        material::of::Ingrediant {
            name: "Coffee".to_string()
        },
        material::of::Ingrediant {
            name: "Sugar".to_string()
        }
    ];
    liqs = vec![
        liquid::is::Mixer {
            name: "Milk".to_string(),
            temp: energy::form::Temperature {
                value: 90,
                unit: "celsius".to_string()
            }
        }];

    let cup_c = brew(&mats, &liqs);
    println!("Cup C is : {}", cup_c);
}

fn brew(materials: &[material::of::Ingrediant], mixers: &[liquid::is::Mixer]) -> String {
    let mut is_milk = false;
    let mut feeling = String::from("");
    let mut taste = String::from("");
    for m in mixers {
        let state = m.state();
        if m.name == "Water" && state == "Cold" {
            feeling = String::from("Ice");
        }
        if m.name == "Milk" && !is_milk {
            is_milk = true;
        }
    }
    for mt in materials {
        if mt.name == "Sugar" {
            taste = String::from("Sweet");
        }
    }
    let mut drink = String::from("Espresso");
    if is_milk {
        drink = String::from("Latte");
    } else {
        if feeling == "Ice" {
            drink = String::from("Americano");
        }
    }
    return format!("a cup of {} {} {}", feeling, taste, drink);
}