use chrono::Duration;

pub const JWT_LIFETIME: Duration = Duration::hours(1);

pub const MIN_USERNAME_LENGHT: usize = 5;
pub const MAX_USERNAME_LENGHT: usize = 20;

pub const MIN_PASS_LENGHT: usize = 5;
pub const MAX_PASS_LENGHT: usize = 64;

pub const AUTH_RATE_LIMIT_CAP: u32 = 3;
pub const AUTH_RATE_LIMIT_LEAK: f64 = 0.1;
