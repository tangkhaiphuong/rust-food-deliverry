use std::any::Any;
use std::collections::HashMap;

use crate::common::AppError;
use crate::modules::restaurant::model::{Restaurant, ENTITY_NAME};
use std::error::Error;
use std::sync::Arc;

pub trait GetRestaurantStorage {
    fn find_data_with_condition(
        &self,
        condition: &HashMap<String, Arc<dyn Any + Send + Sync>>,
    ) -> Result<Option<Restaurant>, Box<dyn Error + Send + Sync>>;
}

pub trait GetRestaurantBusiness {
    fn get_restaurant(&self, id: i32) -> Result<Option<Restaurant>, Box<dyn Error + Send + Sync>>;
}

pub struct DefaultGetRestaurantBusiness {
    pub storage: Arc<dyn GetRestaurantStorage>,
}

impl DefaultGetRestaurantBusiness {
    pub fn new(storage: Arc<dyn GetRestaurantStorage>) -> Self {
        return DefaultGetRestaurantBusiness { storage };
    }
}

impl GetRestaurantBusiness for DefaultGetRestaurantBusiness {
    fn get_restaurant(&self, id: i32) -> Result<Option<Restaurant>, Box<dyn Error + Send + Sync>> {
        let mut condition = HashMap::new();
        let id: Arc<dyn Any + Send + Sync> = Arc::new(id);
        condition.insert("id".to_string(), id);

        return match self.storage.find_data_with_condition(&condition) {
            Ok(value) => Ok(value),
            Err(error) => Err(AppError::new_cannot_get_entity(ENTITY_NAME, error).into()),
        };
    }
}

pub fn new_get_restaurant_business(
    storage: Arc<dyn GetRestaurantStorage>,
) -> Arc<dyn GetRestaurantBusiness> {
    return Arc::new(DefaultGetRestaurantBusiness::new(storage));
}
