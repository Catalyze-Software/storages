use candid::{CandidType, Deserialize, Principal};
use catalyze_shared::{profile::Profile, sort_direction::SortDirection};
use common::Sorter;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ProfileSort {
    CreatedOn(SortDirection),
    UpdatedOn(SortDirection),
}

impl Default for ProfileSort {
    fn default() -> Self {
        ProfileSort::CreatedOn(SortDirection::default())
    }
}

impl Sorter<Principal, Profile> for ProfileSort {
    fn sort(&self, profiles: Vec<(Principal, Profile)>) -> Vec<(Principal, Profile)> {
        let mut profiles = profiles;

        match self {
            ProfileSort::CreatedOn(SortDirection::Asc) => {
                profiles.sort_by(|a, b| a.1.created_on.cmp(&b.1.created_on))
            }
            ProfileSort::CreatedOn(SortDirection::Desc) => {
                profiles.sort_by(|a, b| b.1.created_on.cmp(&a.1.created_on))
            }
            ProfileSort::UpdatedOn(SortDirection::Asc) => {
                profiles.sort_by(|a, b| a.1.updated_on.cmp(&b.1.updated_on))
            }
            ProfileSort::UpdatedOn(SortDirection::Desc) => {
                profiles.sort_by(|a, b| b.1.updated_on.cmp(&a.1.updated_on))
            }
        }
        profiles
    }
}
