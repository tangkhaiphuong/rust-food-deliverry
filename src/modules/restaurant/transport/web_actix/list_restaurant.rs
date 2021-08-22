use actix_web::{web, HttpResponse};

use crate::common::{AppResponse, Paging, DB_TYPE_RESTAURANT};
use crate::component::AppContext;
use crate::modules::restaurant::business::new_list_restaurant_business;
use crate::modules::restaurant::model::Filter;
use crate::modules::restaurant::storage::list::new_list_restaurant_storage;
use crate::modules::restaurant::transport::web_actix::{error, json};

pub fn list_restaurant(
    app_context: web::Data<Box<dyn AppContext>>,
    web::Query(mut paging): web::Query<Paging>,
    web::Query(filter): web::Query<Filter>,
) -> HttpResponse {
    paging = paging.normalize();

    let connection = match app_context.main_connection() {
        Ok(connection) => connection,
        Err(err) => return error(err),
    };

    let storage = new_list_restaurant_storage(connection);
    let business = new_list_restaurant_business(storage);

    let result = business.list_restaurant(&filter, &paging);

    let mut results = match result {
        Err(err) => return error(err),
        Ok(res) => res,
    };
    for item in results.iter_mut() {
        item.base.mask(DB_TYPE_RESTAURANT, 1);
    }

    paging.total = results.len() as i32;

    return json(AppResponse::new(results, paging, filter));
}
