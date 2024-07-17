use candid::{CandidType, Deserialize, Principal};
use catalyze_shared::impl_storable_for;

impl_storable_for!(Principals);

#[derive(Clone, Default, Debug, CandidType, Deserialize)]
pub struct Principals(Vec<Principal>);

impl Principals {
    pub fn new(list: Vec<Principal>) -> Self {
        Self(list)
    }

    pub fn to_vec(&self) -> Vec<Principal> {
        self.0.clone()
    }

    pub fn append(&mut self, other: &mut Vec<Principal>) {
        self.0.append(other);
    }
}

impl From<Vec<Principal>> for Principals {
    fn from(list: Vec<Principal>) -> Self {
        Self::new(list)
    }
}
