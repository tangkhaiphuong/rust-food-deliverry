use std::error::Error;
use std::fmt::Debug;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

pub use create_restaurant::*;
pub use delete_restaurant::*;
pub use get_restaurant::*;
pub use list_restaurant::*;
pub use list_restaurant::*;
pub use update_restaurant::*;

use crate::common::{new_simple_success_response, AppError, AppResponse};

mod create_restaurant;
mod delete_restaurant;
mod get_restaurant;
mod list_restaurant;
mod update_restaurant;

pub(crate) fn error(error: Box<dyn Error + Send + Sync>) -> HttpResponse {
    HttpResponse::BadRequest()
        .content_type("application/json")
        .json(AppError::new_custom_error(
            format!("{:}", &error),
            format!("{:}", &error),
            Some(error),
        ))
}

pub(crate) fn simple_json<'a, T: Serialize + Deserialize<'a> + Debug>(data: T) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(new_simple_success_response(data))
}

fn json<T: Serialize, K: Serialize, P: Serialize>(data: AppResponse<T, K, P>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}
