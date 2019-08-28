use crate::liquid;
use crate::energy;

pub struct Water {
    pub name: String,
    pub temp: energy::form::Temperature
}

impl liquid::Property for Water {
    fn state(&self) -> Option<String> {
        if(self.temp.value <= 0.0) {
            Some("Ice".to_string());
        }
        else if(self.temp.value <= 20.0) {
            Some("Cold".to_string());
        }
        else if(self.temp.value <= 70.0) {
            Some("Warm".to_string());
        }
        else if(self.temp.value <= 100.0) {
            Some("Hot".to_string());
        }
        None;
    }
}