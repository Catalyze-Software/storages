pub trait Filter<T: candid::CandidType>: candid::CandidType + Clone + Send + Sync {
    fn eq_str<S: Into<String>>(a: S, b: S) -> bool {
        a.into().to_lowercase().trim() == b.into().to_lowercase().trim()
    }

    fn filter(&self, value: &T) -> bool; // TODO: add kind
}

pub trait Sorter<K: 'static + candid::CandidType + Ord + Clone + Send + Sync, V: candid::CandidType>:
    candid::CandidType + Clone + Send + Sync
{
    fn sort(&self, values: Vec<(K, V)>) -> Vec<(K, V)>;
}
