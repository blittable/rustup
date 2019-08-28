use crate::liquid;

pub struct Milk {
    pub name: String,
    pub temp: energy::form::Temperature
}

impl liquid::Property for Milk {
    fn state(&self) -> Option<String> {
        if(self.temp.value <= 30.0) {
            None;
        }
        else if(self.temp.value <= 40.0) {
            Some("Cold".to_string());
        }
        else if(self.temp.value <= 65.0) {
            Some("Warm".to_string());
        }
        Some("Hot".to_string());
    }
}