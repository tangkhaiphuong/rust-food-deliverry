use actix_web::{web, HttpRequest, HttpResponse};

use crate::common::{Uid, DB_TYPE_RESTAURANT};
use crate::component::AppContext;
use crate::modules::restaurant::business::new_create_new_restaurant_business;
use crate::modules::restaurant::model::RestaurantCreate;
use crate::modules::restaurant::storage::create::new_restaurant_storage;
use crate::modules::restaurant::transport::web_actix::{error, simple_json};

pub fn create_restaurant(
    app_context: web::Data<Box<dyn AppContext>>,
    web::Json(data): web::Json<RestaurantCreate>,
    _request: HttpRequest,
) -> HttpResponse {
    let connection = match app_context.main_connection() {
        Ok(connection) => connection,
        Err(err) => return error(err),
    };

    let storage = new_restaurant_storage(connection);
    let business = new_create_new_restaurant_business(storage);

    let result = business.create_restaurant(&data);

    return match result {
        Err(err) => error(err),
        Ok(id) => {
            let uid = Uid::new(id as u32, DB_TYPE_RESTAURANT, 1);
            return simple_json(uid.to_string());
        }
    };
}
