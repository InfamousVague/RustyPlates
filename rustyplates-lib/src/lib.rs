pub struct HumanName {
    first: String,
    last: String
}

impl HumanName {
    pub fn first(&self) -> &String {
        &self.first
    }

    pub fn last(&self) -> &String {
        &self.last
    }

    pub fn informal(&self) -> String {
        let mut f = self.first().to_owned();
        let mut l = self.last().to_owned();
        
        format!(
            "{} {}", 
            format!("{}{f}", f.remove(0).to_uppercase()), 
            format!("{}{l}", l.remove(0).to_uppercase())
        )
    }

    pub fn formal(&self) -> String {
        format!("{} {}", self.last, self.first)
    }
}

pub enum WeightMeasurement {
    Kilogram(u128),
    Stone(u128),
    Pounds(u128)
}

pub struct Weight {
    measurement: WeightMeasurement
}

pub struct Human {
    name: HumanName,
    age: u8,
    weight: Weight,
}

impl Human {
    pub fn name(&self) -> &HumanName {
        &self.name
    }
}

pub struct Profile {
    pub name: String,
    pub human: Human,
    // programs
    // statistics
}