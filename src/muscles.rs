#[derive(Debug, Deserialize)]
pub enum Muscle {
    Pectorals,
    Trapezius,
    Rhomboids,
    LatissimusDorsi,
    PosteriorDelts,
    SpinalErectors,
    DeltoidFront,
    DeltoidLateral,
    DeltoidRear,
    Biceps,
    Triceps,
    Forearms,
    Abs,
    Obliques,
    Quads,
    Hamstrings,
    Calves,
    Glutes,
}

#[derive(Debug, Deserialize)]
pub enum MuscleGroup {
    Chest,
    Back,
    Shoulders,
    Arms,
    Core,
    Legs,
    FullBody,
}

// Associated method to get all muscles in a muscle group
impl MuscleGroup {
    pub fn get_muscles(&self) -> Vec<Muscle> {
        match self {
            MuscleGroup::Chest => vec![Muscle::Pectorals],
            MuscleGroup::Back => vec![Muscle::Trapezius, Muscle::Rhomboids, Muscle::PosteriorDelts, Muscle::LatissimusDorsi, Muscle::SpinalErectors],
            MuscleGroup::Shoulders => vec![Muscle::DeltoidFront, Muscle::DeltoidLateral, Muscle::DeltoidRear],
            MuscleGroup::Arms => vec![Muscle::Biceps, Muscle::Triceps, Muscle::Forearms],
            MuscleGroup::Core => vec![Muscle::Abs, Muscle::Obliques],
            MuscleGroup::Legs => vec![Muscle::Quads, Muscle::Hamstrings, Muscle::Calves, Muscle::Glutes],
            MuscleGroup::FullBody => vec![
                Muscle::Pectorals,
                Muscle::Trapezius,
                Muscle::Rhomboids,
                Muscle::PosteriorDelts,
                Muscle::LatissimusDorsi,
                Muscle::SpinalErectors,
                Muscle::DeltoidFront,
                Muscle::DeltoidLateral,
                Muscle::DeltoidRear,
                Muscle::Biceps,
                Muscle::Triceps,
                Muscle::Forearms,
                Muscle::Abs,
                Muscle::Obliques,
                Muscle::Quads,
                Muscle::Hamstrings,
                Muscle::Calves,
                Muscle::Glutes,
            ],
        }
    }
}