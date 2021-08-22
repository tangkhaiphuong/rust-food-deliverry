use actix_web::{web, HttpRequest, HttpResponse};

use crate::common::Uid;
use crate::component::AppContext;
use crate::modules::restaurant::business::new_delete_restaurant_business;
use crate::modules::restaurant::storage::delete::new_delete_restaurant_storage;
use crate::modules::restaurant::transport::web_actix::{error, simple_json};

pub fn delete_restaurant(
    app_context: web::Data<Box<dyn AppContext>>,
    _request: HttpRequest,
    web::Path((id,)): web::Path<(String,)>,
) -> HttpResponse {
    let uid = Uid::from_base58(id);

    let uid = match uid {
        Ok(uid) => uid,
        Err(err) => return error(err),
    };

    let connection = match app_context.main_connection() {
        Ok(connection) => connection,
        Err(err) => return error(err),
    };

    let storage = new_delete_restaurant_storage(connection);
    let business = new_delete_restaurant_business(storage);

    let result = business.delete_restaurant(uid.local_id() as i32);

    return match result {
        Err(err) => error(err),
        Ok(_) => simple_json(true),
    };
}
