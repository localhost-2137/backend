use std::env;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MAPS_API_KEY: String = {
        env::var("GOOGLE_MAPS_API_KEY").unwrap()
    };
}
