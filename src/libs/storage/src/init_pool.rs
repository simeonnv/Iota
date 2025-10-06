use crate::Error;
use std::fs;
use surrealdb::{
    Surreal,
    engine::local::{Db, RocksDb},
};

pub async fn init_pool(storage_name: &'static str) -> Result<Surreal<Db>, Error> {
    let mut data_dir = dirs::data_dir().ok_or(Error::UnableToFindAppData())?;
    data_dir.push("Iota");
    fs::create_dir_all(&data_dir).map_err(|e| Error::UnableToCreateStorageDir(e.to_string()))?;

    let pool =
        Surreal::new::<RocksDb>(format!("{}/{}", data_dir.to_string_lossy(), storage_name)).await?;

    pool.use_ns("Iota").use_db(storage_name).await?;

    Ok(pool)
}
