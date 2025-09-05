use error::Error;

pub fn insure_len(input: &String, min: usize, max: usize) -> Result<(), Error> {
    if input.len() >= min && input.len() < max {
        return Ok(());
    } else {
        Err(Error::BadRequest(format!(
            "String size must be between {} and {}, but got {}",
            min,
            max,
            input.len()
        )))
    }
}
