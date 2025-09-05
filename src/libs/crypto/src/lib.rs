use argon2::Params;

mod rand_string;
pub use rand_string::rand_string;

mod argon2_hash;
pub use argon2_hash::argon2_hash;

mod compare_argon2_hash;
pub use compare_argon2_hash::compare_argon2_hash;

pub const ARGON2_PARAMS: Result<Params, argon2::Error> = Params::new(
    8192, // Memory cost
    1,    // Iterations
    2,    // Parallelism
    None, // Optional output length (None uses default)
);
