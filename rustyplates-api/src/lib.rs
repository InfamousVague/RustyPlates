use rustyplates_lib::{Human, Profile};

pub enum APIError {
    CreateProfile(String)
}

pub fn create_profile(human: Human) -> Result<Profile, APIError> {
    // Make sure profile doesn't exist with this name already.
    // Err(APIError::CreateProfile("Couldn't create profile, an existing profile exists with the same name."))
    Ok(Profile {
        name: human.name().first().to_string(),
        human
    })
}

pub fn list_profiles() -> Vec<Profile> {
    vec![] // TODO
}