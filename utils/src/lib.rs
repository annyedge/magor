use dotenv::dotenv;
use std::env;

pub fn get_env_var(key: &str) -> Option<String> {
    dotenv().ok();

    match env::var(key) {
        Ok(value) => Some(value),
        Err(_) => {
            eprintln!("Warning: Environment variable '{}' is not set.", key);
            None
        }
    }
}
