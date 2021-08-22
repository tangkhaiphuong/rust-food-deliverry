use std::error::Error;

use lazy_static::lazy_static;

pub use filter::*;
pub use restaurant::*;
pub use restaurant_create::*;
pub use restaurant_update::*;

mod filter;
mod restaurant;
mod restaurant_create;
mod restaurant_update;
pub mod schema;

pub const ENTITY_NAME: &str = "Restaurant";

lazy_static! {
    pub static ref ERR_ADDRESS_CANNOT_BE_BLANK: Box<dyn Error + Sync + Send> = {
        return "name cannot be blank".to_string().into();
    };
    pub static ref ERR_NAME_CANNOT_BE_BLANK: Box<dyn Error + Sync + Send> = {
        return "address cannot be blank".to_string().into();
    };
}
