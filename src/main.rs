#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::{fs, time::Duration};

use muscles::MuscleGroup;
use equipment::Equipment;
use plan::Plan;

pub mod muscles;
pub mod equipment;
pub mod plan;

#[derive(Debug, Deserialize)]
pub enum Kind {
    RepititionSets(Vec<u8>), // The vec here represents sets of repititions of an exercise
    Duration(Vec<Duration>), // The vec here represents multiple durations of an exercise
    Rest,
}

#[derive(Debug, Deserialize)]
pub struct Workout {
    pub name: String,
    pub kind: Kind,
    pub muscle_groups: Vec<MuscleGroup>,
    pub weight: u16,
    pub equipment: Vec<Equipment>,
}

fn main() {
    // Load workout data from workouts.json (JSON format)
    let json_content = fs::read_to_string("assets/workouts/bodyweight.json").expect("Failed to read bodyweight.json");
    let decoded_workouts: Vec<Workout> = serde_json::from_str(&json_content).expect("Failed to parse JSON");

    // Print the loaded workout data
    for workout in &decoded_workouts {
        println!("{:?}", workout);
    }

    // Load workout plan data from workout_plan.json (JSON format)
    let plan_json_content = fs::read_to_string("assets/example_plan.json").expect("Failed to read example_plan.json");
    let decoded_plan: Plan<Workout> = serde_json::from_str(&plan_json_content).expect("Failed to parse JSON");

    // Print the loaded workout plan data
    println!("Workout Plan: {:?}", decoded_plan);
}
