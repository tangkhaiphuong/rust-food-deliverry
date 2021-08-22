use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};

use crate::common::{Requester, Uid};
use crate::component::AppContext;
use crate::modules::restaurant::business::new_update_restaurant_business;
use crate::modules::restaurant::model::RestaurantUpdate;
use crate::modules::restaurant::storage::update::new_update_restaurant_storage;
use crate::modules::restaurant::transport::web_actix::{error, simple_json};
use crate::modules::user::model::User;

pub fn update_restaurant(
    app_context: web::Data<Box<dyn AppContext>>,
    data: web::Json<RestaurantUpdate>,
    web::Path((id,)): web::Path<(String,)>,
    _request: HttpRequest,
) -> HttpResponse {
    let uid = Uid::from_base58(id);

    let uid = match uid {
        Ok(uid) => uid,
        Err(err) => return error(err),
    };

    let requester: Arc<dyn Requester> = Arc::new(User::default());

    let connection = match app_context.main_connection() {
        Ok(connection) => connection,
        Err(err) => return error(err),
    };

    let storage = new_update_restaurant_storage(connection);
    let business = new_update_restaurant_business(storage, requester);

    let result = business.update_restaurant(uid.local_id() as i32, &data);

    return match result {
        Err(err) => error(err),
        Ok(_) => simple_json(true),
    };
}
