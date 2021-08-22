use std::any::Any;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::sync::Arc;

use diesel::dsl::sql;
use diesel::{QueryDsl, RunQueryDsl};

use crate::common::MainPooledConnection;
use crate::modules::restaurant::business::GetRestaurantStorage;
use crate::modules::restaurant::model::schema::restaurants::table;
use crate::modules::restaurant::model::schema::RestaurantEntity;
use crate::modules::restaurant::model::Restaurant;
use crate::modules::restaurant::storage::SqlStorage;

impl GetRestaurantStorage for SqlStorage {
    fn find_data_with_condition(
        &self,
        condition: &HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Result<Option<Restaurant>, Box<dyn Error + Send + Sync>> {
        let sql_str = condition
            .iter()
            .map(|(key, value)| {
                let value = value.clone().downcast::<String>().unwrap_or(Arc::from(
                    value
                        .clone()
                        .downcast::<i32>()
                        .unwrap_or_default()
                        .to_string(),
                ));
                return format!("{} = {}", key, value);
            })
            .collect::<Vec<_>>()
            .join(" AND ");

        let entity = table
            .filter(sql(&sql_str))
            .first::<RestaurantEntity>(&self.connection);

        let entity = entity.map_err(|e| Box::new(e))?;

        let entity = Restaurant::try_from(entity)?;

        return Ok(Some(entity));
    }
}

pub fn new_get_restaurant_storage(
    connection: MainPooledConnection,
) -> Arc<dyn GetRestaurantStorage> {
    return Arc::new(SqlStorage::new(connection));
}
