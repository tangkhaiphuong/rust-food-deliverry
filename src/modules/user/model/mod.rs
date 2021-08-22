use lazy_static::lazy_static;

pub use user::*;

mod user;

pub const ENTITY_NAME: &str = "User";

lazy_static! {
    // pub static ref ERR_ADDRESS_CANNOT_BE_BLANK: Box<dyn Error + Sync + Send> = {
    //     return AppError::new_custom_error() ;
    // };

}
