use async_trait::async_trait;
use deadpool_postgres::{Config, Runtime, Pool};
use deadpool_postgres::tokio_postgres::NoTls;
use crate::error::Error;

pub struct DaoImpl {
    connection_pool: Pool
}

impl DaoImpl{
    pub fn new(dbname: &str) -> Self {
        let mut cfg = Config::new();
        cfg.dbname = Some(dbname.to_string());
        let connection_pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
        Self {
            connection_pool
        }
    }
}

#[async_trait]
trait Dao {
    async fn get_items(&self) -> Result<Vec<String>, Error>;

    async fn add_item(&self, _: &str) -> Result<(), Error>;
}
#[async_trait]
impl Dao for DaoImpl {
    async fn get_items(&self) -> Result<Vec<String>, Error>{
        let _client = self.connection_pool.get().await;
        unimplemented!()
    }

    async fn add_item(&self, _: &str) -> Result<(), Error> {
        unimplemented!()
    }
}