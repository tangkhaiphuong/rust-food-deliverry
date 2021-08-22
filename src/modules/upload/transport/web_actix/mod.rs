use std::error::Error;
use std::fmt::Debug;

use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::common::{new_simple_success_response, AppError, AppResponse};

mod upload_image;

fn error(error: Box<dyn Error + Send + Sync>) -> HttpResponse {
    HttpResponse::BadRequest()
        .content_type("application/json")
        .json(AppError::new_custom_error(
            format!("{:}", &error),
            format!("{:}", &error),
            Some(error),
        ))
}

fn simple_json<'a, T: Serialize + Deserialize<'a> + Debug>(data: T) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(new_simple_success_response(data))
}

fn json<T: Serialize, K: Serialize, P: Serialize>(data: AppResponse<T, K, P>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}
