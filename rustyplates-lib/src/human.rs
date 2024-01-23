use crate::weight::Weight;

/// Represents the parts of a given humans name
pub struct HumanName {
    first: String,
    last: String
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
    pub fn age(&self) -> &u8 {
        &self.age
    }
    pub fn weight(&self) -> &Weight {
        &self.weight
    }
}

impl HumanName {
    /// Returns the first name of the human
    pub fn first(&self) -> &String {
        &self.first
    }

    /// Returns the last name of the human
    pub fn last(&self) -> &String {
        &self.last
    }

    /// Auto capitalize the first letter of the first and last name, combine informally (e.g. John Doe)
    pub fn informal(&self) -> String {
        let mut f = self.first().to_owned();
        let mut l = self.last().to_owned();
        
        format!(
            "{} {}", 
            format!("{}{f}", f.remove(0).to_uppercase()), 
            format!("{}{l}", l.remove(0).to_uppercase())
        )
    }

    /// Uppercase both parts of the name and arrange formally (e.g. DOE, JOHN)
    pub fn formal(&self) -> String {
        format!("{}, {}", self.last.to_uppercase(), self.first.to_uppercase())
    }
}