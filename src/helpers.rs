pub fn is_debug_mode() -> bool {
    match std::env::var("DEBUG") {
        Ok(val) => val.parse::<bool>().unwrap_or(false),
        Err(_) => false,
    }
}
