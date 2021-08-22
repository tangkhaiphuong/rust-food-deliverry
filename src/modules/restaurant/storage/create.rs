use std::convert::TryFrom;
use std::error::Error;
use std::sync::Arc;

use diesel::insert_into;

use crate::common::{AppError, MainPooledConnection};
use crate::diesel::RunQueryDsl;
use crate::modules::restaurant::business::CreateRestaurantStorage;
use crate::modules::restaurant::model::schema::restaurants;
use crate::modules::restaurant::model::schema::RestaurantEntity;
use crate::modules::restaurant::model::{RestaurantCreate, ENTITY_NAME};
use crate::modules::restaurant::storage::SqlStorage;

impl CreateRestaurantStorage for SqlStorage {
    fn create(&self, data: &RestaurantCreate) -> Result<i32, Box<dyn Error + Send + Sync>> {
        let entity = RestaurantEntity::try_from(data)?;
        let result = insert_into(restaurants::dsl::restaurants)
            .values(&entity)
            .execute(&self.connection);

        let result = result.map_err(|e| Box::new(e))?;

        if result == 0 {
            return Err(Box::new(AppError::new_cannot_create_entity(
                ENTITY_NAME,
                None,
            )));
        }

        return Ok(entity.id.unwrap_or_default());
    }
}

pub fn new_restaurant_storage(
    connection: MainPooledConnection,
) -> Arc<dyn CreateRestaurantStorage> {
    return Arc::new(SqlStorage::new(connection));
}
