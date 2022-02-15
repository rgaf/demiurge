mod stateful_rand;
mod stateless_rand;

pub use stateful_rand::StatefulRand;
pub use stateless_rand::StatelessRand;

use std::str::FromStr;
use std::time::SystemTime;

pub fn get_current_seed() -> u64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => {
            let secs = duration.as_secs();
            let nanos = duration.subsec_nanos() as u64;
            let value = StatelessRand::from_seed(secs).hash_1u64(nanos);

            value
        },

        Err(_) => panic!("Error while reading system time")
    }
}

pub fn parse_seed(string: &str) -> u64 {
    i64::from_str(string).map(|signed_int| signed_int as u64).or_else(|_| {
        u64::from_str(string)
    }).unwrap_or_else(|_| {
        StatelessRand::from_seed(0x5712_5EED_DE7EC7ED).hash_bytes(string.as_bytes())
    })
}
