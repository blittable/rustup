//Basics of Traits - Homework

fn main() {

    //Create a struct that has String field

    //Create a trait that returns a String

    //Implement the trait for your struct

    //Call it from main()

    //Extra Credit:
    // 1) create a String value
    // 2) print the same string value returned from the trait above, but without using a struct, hint ::

    let samsung: Mobiles = Mobiles {
        band: "Samsung".to_string(),
        price: 499.99,
    };

    let band_text = samsung.show_band();
    println!("Mobile: {}", band_text);

    //Extra
    println!("Extra mobile: {}", self::Mobiles::show_band(&samsung));
    
    //Extra2
    let iphone :String = "iPhone".to_string();
    let iphone_price :f32 = 555.55;
    println!("Extra2 mobile: {}", self::Mobiles::show_extra_band(iphone, iphone_price))
}

struct Mobiles {
    band: String,
    price: f32,
}

trait ShowBand {
    fn show_band(&self) -> String;
    fn show_extra_band(manual_band: String, manual_price: f32) -> String;
}

impl ShowBand for Mobiles {
    fn show_band(&self) -> String {
        return format!("band {} price {}",self.band, self.price);
    }

    fn show_extra_band(manual_band: String, manual_price: f32) -> String {
        return format!("band {} price {}",manual_band, manual_price);;
    }
}