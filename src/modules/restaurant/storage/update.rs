use std::any::Any;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::sync::Arc;

use diesel::update;
use diesel::{QueryDsl, RunQueryDsl};

use crate::common::MainPooledConnection;
use crate::diesel::ExpressionMethods;
use crate::modules::restaurant::business::UpdateRestaurantStorage;
use crate::modules::restaurant::model::schema::restaurants;
use crate::modules::restaurant::model::schema::RestaurantEntity;
use crate::modules::restaurant::model::RestaurantUpdate;
use crate::modules::restaurant::storage::SqlStorage;

impl UpdateRestaurantStorage for SqlStorage {
    fn update(
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

        let entity = RestaurantEntity::try_from(data)?;

        let result = update(restaurants::table.filter(restaurants::id.eq(id)))
            .set(entity)
            .execute(&self.connection);

        let result = match result {
            Err(err) => return Err(err.into()),
            Ok(mm) => mm,
        };

        if result == 0 {
            return Ok(false);
        }

        return Ok(true);
    }
}

pub fn new_update_restaurant_storage(
    connection: MainPooledConnection,
) -> Arc<dyn UpdateRestaurantStorage> {
    return Arc::new(SqlStorage::new(connection));
}
