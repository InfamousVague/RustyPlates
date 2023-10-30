#[derive(Debug, Deserialize)]
pub struct Plan<Activity> {
    monday:     Vec<Activity>,
    tuesday:    Vec<Activity>,
    wednesday:  Vec<Activity>, 
    thursday:   Vec<Activity>,
    friday:     Vec<Activity>,
    saturday:   Vec<Activity>,
    sunday:     Vec<Activity>
}

#[derive(Debug, Deserialize)]
pub enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday
}

impl<Activity> Plan<Activity> {
    pub fn get_workout_by_day(self, day: Day) -> Vec<Activity> {
        match day {
            Day::Monday => self.monday,
            Day::Tuesday => self.tuesday,
            Day::Wednesday => self.wednesday,
            Day::Thursday => self.thursday,
            Day::Friday => self.friday,
            Day::Saturday => self.saturday,
            Day::Sunday => self.sunday,
        }
    }

    pub fn set_workout_for_day(&mut self, day: Day, workout: Vec<Activity>) {
        match day {
            Day::Monday => self.monday = workout,
            Day::Tuesday => self.tuesday = workout,
            Day::Wednesday => self.wednesday = workout,
            Day::Thursday => self.thursday = workout,
            Day::Friday => self.friday = workout,
            Day::Saturday => self.saturday = workout,
            Day::Sunday => self.sunday = workout,
        }
    }
}