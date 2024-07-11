use candid::{CandidType, Deserialize};
use catalyze_shared::{date_range::DateRange, profile::Profile};
use common::Filter;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileFilter {
    Username(String),
    DisplayName(String),
    FirstName(String),
    LastName(String),
    Email(String),
    City(String),
    StateOrProvince(String),
    Country(String),
    UpdatedOn(DateRange),
    Skill(u32),
    Interest(u32),
    Cause(u32),
    CreatedOn(DateRange),
}

impl Filter<Profile> for ProfileFilter {
    fn filter(&self, value: &Profile) -> bool {
        match self {
            ProfileFilter::Username(username) => Self::eq_str(&value.username, username),
            ProfileFilter::DisplayName(display_name) => {
                Self::eq_str(&value.display_name, display_name)
            }
            ProfileFilter::FirstName(first_name) => Self::eq_str(&value.first_name, first_name),
            ProfileFilter::LastName(last_name) => Self::eq_str(&value.last_name, last_name),
            ProfileFilter::Email(email) => Self::eq_str(&value.email, email),
            ProfileFilter::City(city) => Self::eq_str(&value.city, city),
            ProfileFilter::StateOrProvince(state_or_province) => {
                Self::eq_str(&value.state_or_province, state_or_province)
            }
            ProfileFilter::Country(country) => Self::eq_str(&value.country, country),
            ProfileFilter::UpdatedOn(updated_on) => updated_on.is_within(value.updated_on),
            ProfileFilter::Skill(skill) => value.skills.contains(skill),
            ProfileFilter::Interest(interest) => value.interests.contains(interest),
            ProfileFilter::Cause(cause) => value.causes.contains(cause),
            ProfileFilter::CreatedOn(created_on) => created_on.is_within(value.created_on),
        }
    }
}
