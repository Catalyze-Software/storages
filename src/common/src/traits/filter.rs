pub trait Filter<T: candid::CandidType>: candid::CandidType + Clone + Send + Sync {
    fn filter(&self, value: &T) -> bool;
}

pub type Filters<T> = Vec<Box<dyn Filter<T>>>;
