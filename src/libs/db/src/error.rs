use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("pool init error!: {0}")]
    PoolInitError(String),
}
