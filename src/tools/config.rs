use dotenv::dotenv;
use std::env;

#[allow(dead_code)]
fn init_env() {
    dotenv().ok();
}

/// Retrieves an environment variable as an integer or returns a default value.
pub fn get_env_var_as_int(key: &str) -> i32 {
    let default = 0;
    env::var(key)
        .unwrap_or(default.to_string())
        .parse()
        .unwrap_or_else(|_| panic!("{} must be an integer", key))
}

/// Retrieves an environment variable as a string or returns an empty string if not found.
pub fn get_env_var_as_string(key: &str) -> String {
    env::var(key).unwrap_or_default()
}
