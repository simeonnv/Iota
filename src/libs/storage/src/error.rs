use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("pool init error!: {0}")]
    PoolInitError(#[from] surrealdb::Error),

    #[error("Unable to find appdata while initing storage pool!")]
    UnableToFindAppData(),

    #[error("Unable to create storage dir: {0}")]
    UnableToCreateStorageDir(String),
}
