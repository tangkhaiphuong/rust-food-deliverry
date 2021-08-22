use actix_web::{web, HttpRequest, HttpResponse};

use crate::common::{AppError, Uid, DB_TYPE_RESTAURANT};
use crate::component::AppContext;
use crate::modules::restaurant::business::new_get_restaurant_business;
use crate::modules::restaurant::model::Restaurant;
use crate::modules::restaurant::storage::get::new_get_restaurant_storage;
use crate::modules::restaurant::transport::web_actix::{error, simple_json};

pub fn get_restaurant(
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
        Err(err) => return error(AppError::new_invalid_request(err).into()),
    };

    let storage = new_get_restaurant_storage(connection);
    let business = new_get_restaurant_business(storage);

    let result = business.get_restaurant(uid.local_id() as i32);

    let result = match result {
        Err(err) => return error(err),
        Ok(res) => res,
    };

    let mut result = match result {
        None => return simple_json::<Option<Restaurant>>(None),
        Some(res) => res,
    };

    result.base.mask(DB_TYPE_RESTAURANT, 1);

    return simple_json(result);
}
