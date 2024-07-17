use candid::{CandidType, Deserialize, Principal};
use catalyze_shared::impl_storable_for;

impl_storable_for!(Shard);

#[derive(Clone, CandidType, Deserialize)]
pub struct Shard {
    id: Principal,
    filled: bool,
}

impl Shard {
    pub fn new(id: Principal) -> Self {
        Self { id, filled: false }
    }

    pub fn id(&self) -> Principal {
        self.id
    }

    pub fn filled(&self) -> bool {
        self.filled
    }

    pub fn set_filled(&mut self, filled: bool) {
        self.filled = filled;
    }
}
