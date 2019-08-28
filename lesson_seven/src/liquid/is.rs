use crate::energy;
use crate::liquid;

pub struct Mixer {
    pub name: String,
    pub temp: energy::form::Temperature
}

impl liquid::Property for Mixer {
    fn state(&self) -> String {
        return match self.temp.value/30 {
            0 => "Ice".to_string(),
            1 => "Cold".to_string(),
            _ => "Hot".to_string()
        }
    }
}