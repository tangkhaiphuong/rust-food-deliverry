use std::sync::Arc;

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

use crate::common::MainPooledConnection;
use crate::component::upload_provider::UploadProvider;
use crate::component::EnvSetting;
use std::error::Error;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub struct DefaultAppContext {
    // connection: MySqlPooledConnection,
    // upload_provider: Box<dyn UploadProvider>,
    setting: EnvSetting,
    pool: Pool,
}

pub trait AppContext {
    fn main_connection(&self) -> Result<MainPooledConnection, Box<dyn Error + Send + Sync>>;
    fn upload_provider(&self) -> Arc<dyn UploadProvider>;
    fn secret_key(&self) -> String;
}

impl DefaultAppContext {
    pub fn new(setting: EnvSetting) -> Self {
        let manager = ConnectionManager::<MysqlConnection>::new(setting.connection_string.as_str());

        let pool: Pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        return DefaultAppContext { setting, pool };
    }
}

impl AppContext for DefaultAppContext {
    fn main_connection(&self) -> Result<MainPooledConnection, Box<dyn Error + Send + Sync>> {
        let conn: PooledConnection<ConnectionManager<MysqlConnection>> = self.pool.get()?;
        return Ok(conn);
    }

    fn upload_provider(&self) -> Arc<dyn UploadProvider> {
        //return self.upload_provider.deref();
        todo!()
    }

    fn secret_key(&self) -> String {
        //  return self.secret.clone();
        todo!()
    }
}
