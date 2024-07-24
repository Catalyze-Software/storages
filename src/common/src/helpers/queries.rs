pub fn icts_name() -> String {
    env!("CARGO_PKG_NAME").to_string()
}

pub fn icts_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
