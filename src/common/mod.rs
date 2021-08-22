use std::error::Error;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::MysqlConnection;
use lazy_static::lazy_static;

pub use self::app_error::*;
pub use self::app_response::*;
pub use self::image::*;
pub use self::paging::*;
pub use self::paging::*;
pub use self::random::*;
pub use self::sql_model::*;
pub use self::uid::*;

mod app_error;
mod app_response;
mod image;
mod paging;
mod random;
mod sql_model;
mod uid;

pub const DB_TYPE_RESTAURANT: u32 = 1;
pub const DB_TYPE_USER: u32 = 2;

pub type MainPooledConnection = PooledConnection<ConnectionManager<MysqlConnection>>;

lazy_static! {
    pub static ref ERR_DATA_NOT_FOUND: Box<dyn Error + Sync + Send> = {
        return "data not found".to_string().into();
    };
    pub static ref ERR_RECORD_NOT_FOUND: Box<dyn Error + Sync + Send> = {
        return "record not found".to_string().into();
    };
}

pub trait Requester {
    fn user_id(&self) -> i32;
    fn email(&self) -> String;
    fn role(&self) -> String;
}
