use crate::error::Error;
use async_trait::async_trait;
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Config, GenericClient, Pool, Runtime};

#[derive(Clone)]
pub struct DaoImpl {
    connection_pool: Pool,
}

impl DaoImpl {
    pub fn new(
        hostname: String,
        dbname: String,
        username: String,
        password: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut cfg = Config::new();
        cfg.host = Some(hostname);
        cfg.dbname = Some(dbname);
        cfg.user = Some(username);
        cfg.password = Some(password);
        let connection_pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;
        Ok(Self { connection_pool })
    }
}

#[async_trait]
pub trait Dao: 'static + Clone + Send + Sync {
    async fn get_items(&self) -> Result<Vec<String>, Error>;

    async fn add_item(&self, _: &str) -> Result<(), Error>;
}
#[async_trait]
impl Dao for DaoImpl {
    async fn get_items(&self) -> Result<Vec<String>, Error> {
        let client = self.connection_pool.get().await?;
        Ok(client
            .query("SELECT message FROM messages", &[])
            .await?
            .iter()
            .map(|row| row.get(0))
            .collect())
    }

    async fn add_item(&self, message: &str) -> Result<(), Error> {
        let client = self.connection_pool.get().await?;
        client
            .execute("INSERT INTO messages (message) VALUE ($1)", &[&message])
            .await?;
        Ok(())
    }
}

#[cfg(test)]
#[derive(Clone, Default)]
pub struct DaoTest {
    pub contents: std::sync::Arc<std::sync::Mutex<Vec<String>>>,
}

#[cfg(test)]
#[async_trait]
impl Dao for DaoTest {
    async fn get_items(&self) -> Result<Vec<String>, Error> {
        Ok(self.contents.lock().unwrap().clone())
    }

    async fn add_item(&self, message: &str) -> Result<(), Error> {
        self.contents.lock().unwrap().push(message.to_owned());
        Ok(())
    }
}
