use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRate {
    pub xdr_permyriad_per_icp: u64,
    pub timestamp_seconds: u64,
}

#[derive(CandidType, Deserialize)]
pub struct IcpXdrConversionRateResponse {
    pub certificate: Vec<u8>,
    pub data: IcpXdrConversionRate,
    pub hash_tree: Vec<u8>,
}
