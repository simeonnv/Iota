use argon2::Params;

mod rand_string;
pub use rand_string::rand_string;

pub mod hashing;
pub mod sign;

pub const ARGON2_PARAMS: Result<Params, argon2::Error> = Params::new(
    8192, // Memory cost
    1,    // Iterations
    2,    // Parallelism
    None, // Optional output length (None uses default)
);
