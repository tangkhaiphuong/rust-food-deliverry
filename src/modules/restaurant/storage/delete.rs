use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use diesel::update;
use diesel::{QueryDsl, RunQueryDsl};

use crate::common::MainPooledConnection;
use crate::diesel::ExpressionMethods;
use crate::modules::restaurant::business::DeleteRestaurantStorage;
use crate::modules::restaurant::model::schema::restaurants;
use crate::modules::restaurant::model::RestaurantUpdate;
use crate::modules::restaurant::storage::SqlStorage;

impl DeleteRestaurantStorage for SqlStorage {
    fn delete(
        &self,
        condition: &HashMap<String, Arc<dyn Any + Send + Sync>>,
        data: &RestaurantUpdate,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let id = *condition
            .get("id")
            .ok_or("Id not found")?
            .clone()
            .downcast::<i32>()
            .map_err(|_| "Cannot cast id")?;

        let id = restaurants::id.eq(id);

        let result = update(restaurants::table.filter(id))
            .set(restaurants::status.eq(data.status as i32))
            .execute(&self.connection);

        let result = result.map_err(|e| Box::new(e))?;

        if result == 0 {
            return Ok(false);
        }

        return Ok(true);
    }
}

pub fn new_delete_restaurant_storage(
    connection: MainPooledConnection,
) -> Arc<dyn DeleteRestaurantStorage> {
    return Arc::new(SqlStorage::new(connection));
}
