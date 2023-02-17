use std::time::Duration;

pub const IP_SERVERS: &str = "127.0.0.1:8080";
pub const MAX_SEATS: usize = 10;
pub const MAX_USERNAME_LENGTH: u32 = 16;
pub const NUM_THREADS: usize = 3;
pub const CONNECTION_RETRIES: u16 = 3;
pub const PAYMENT_LIMIT_TIME: Duration = Duration::from_secs(25);
