use std::error::Error;
use std::str::FromStr;
use std::sync::Arc;

use diesel::{QueryDsl, RunQueryDsl};
use std::convert::TryFrom;

use crate::common::{MainPooledConnection, Paging};
use crate::diesel::ExpressionMethods;
use crate::modules::restaurant::business::ListRestaurantStorage;
use crate::modules::restaurant::model::schema::restaurants;
use crate::modules::restaurant::model::schema::RestaurantEntity;
use crate::modules::restaurant::model::{Filter, Restaurant};
use crate::modules::restaurant::storage::SqlStorage;

impl ListRestaurantStorage for SqlStorage {
    fn list_data_with_condition(
        &self,
        filter: &Filter,
        paging: &Paging,
    ) -> Result<Vec<Restaurant>, Box<dyn Error + Send + Sync>> {
        let offset = (paging.page - 1) * paging.limit;

        let query = restaurants::table
            .offset(offset.into())
            .limit(paging.limit.into());

        let entities = match filter.user_id {
            None => query.load::<RestaurantEntity>(&self.connection),
            Some(ref id) => match i32::from_str(&id) {
                Err(_) => query.load::<RestaurantEntity>(&self.connection),
                Ok(ii) => query
                    .filter(restaurants::owner_id.eq(ii))
                    .load::<RestaurantEntity>(&self.connection),
            },
        };

        let entities = entities.map_err(|e| Box::new(e))?;

        return Ok(entities
            .into_iter()
            .map(Restaurant::try_from)
            .map(|c| c.unwrap())
            .collect());
    }
}

pub fn new_list_restaurant_storage(
    connection: MainPooledConnection,
) -> Arc<dyn ListRestaurantStorage> {
    return Arc::new(SqlStorage::new(connection));
}
