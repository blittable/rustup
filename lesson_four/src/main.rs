//Write some code that does the following:
//1: uses an Option<T>
//2: uses Result<T, E> type
//3: uses 'match'
struct Person{
    name: String,
    weight_kg: Option<f32>,
    height_m: Option<f32>
}

trait Action{
    fn get_name(&self) -> String;
}

impl Action for Person{
    fn get_name(&self) -> String{
        self.name.to_owned()
    }
}

fn get_bmi(guy: Person) -> Option<f32>{
    match guy.height_m{
        None => None,
        Some(h) => {
            match guy.weight_kg{
                None => None,
                Some(w) => Some((w / ( h * h )) as f32)
            }
        }
    }
}

fn print_weight_class(i: f32){
    if(i < 18.5){
        println!("Weight class is underweight.");
    }
    else if(i >= 18.5 && i <25.0){
        println!("Weight class is normal.");
    }
    else if(i >= 25.0 && i < 30.0){
        println!("Weight class is overweight.");
    }
    else if(i >30.0){
        println!("Weight class is very overweight.");
    }
}

fn get_bmi_result(guy: Person) -> Result<f32,String>{
     match guy.height_m {
        None => Err("IncorrectData".to_string()),
        Some(h) => {
            if(h == 0.0){
                Err("DivisionByZero".to_string())
            }else{
                match guy.weight_kg{
                    None => Err("IncorrectData".to_string()),
                    Some(w) =>
                    {
                        Ok(w / ( h * h ))
                    } 
                }
            }
            
        }
    }
}

fn main() {
    //Option<T>
    println!("=> Option<T> <=");
    let someone_correct_info: Person = Person{
        name: "Beree".to_string(),
        weight_kg: Some(55.0),
        height_m: Some(1.59)
    };

    let someone_correct_info_name = someone_correct_info.get_name();
    let someone_correct_info_bmi = get_bmi(someone_correct_info);

    println!("**** {}'s Info ***",someone_correct_info_name);
    match someone_correct_info_bmi {
        None => println!("No personal data"),
        Some(i) => { 
            println!("BMI is {:?}", i);
            print_weight_class(i);
        }
    }

    println!("");

    //Result<T, E>
    println!("=> Result<T, E> <=");
    let someone_incorrect_info: Person = Person{
        name: "Maurice".to_string(),
        weight_kg: Some(55.0),
        height_m: Some(0.0)
    };

    let someone_incorrect_info_name = someone_incorrect_info.get_name();
    let someone_incorrect_info_bmi = get_bmi_result(someone_incorrect_info);

    println!("**** {}'s Info ***",someone_incorrect_info_name);
    match someone_incorrect_info_bmi {
        Ok(i)  => {
            println!("BMI is {:?}", i);
            print_weight_class(i);
        },
        Err(e) => println!("Error - {}", e)
    }

}
