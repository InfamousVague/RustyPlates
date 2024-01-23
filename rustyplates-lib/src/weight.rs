pub enum WeightMeasurement {
    Kilogram(u128),
    Stone(u128),
    Pounds(u128)
}

pub struct Weight {
    measurement: WeightMeasurement
}