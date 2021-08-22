use std::any::Any;
use std::collections::HashMap;

use crate::common::AppError;
use crate::modules::restaurant::business::GetRestaurantStorage;
use crate::modules::restaurant::model::RestaurantUpdate;
use crate::modules::restaurant::model::ENTITY_NAME;
use std::error::Error;
use std::sync::Arc;

pub trait DeleteRestaurantStorage: GetRestaurantStorage {
    fn delete(
        &self,
        condition: &HashMap<String, Arc<dyn Any + Send + Sync>>,
        data: &RestaurantUpdate,
    ) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

pub trait DeleteRestaurantBusiness {
    fn delete_restaurant(&self, id: i32) -> Result<(), Box<dyn Error + Send + Sync>>;
}

pub struct DefaultDeleteRestaurantBusiness {
    pub storage: Arc<dyn DeleteRestaurantStorage>,
}

impl DefaultDeleteRestaurantBusiness {
    fn new(storage: Arc<dyn DeleteRestaurantStorage>) -> Self {
        return DefaultDeleteRestaurantBusiness { storage };
    }
}

impl DeleteRestaurantBusiness for DefaultDeleteRestaurantBusiness {
    fn delete_restaurant(&self, id: i32) -> Result<(), Box<dyn Error + Send + Sync>> {
        let mut condition = HashMap::new();
        let id: Arc<dyn Any + Send + Sync> = Arc::new(id);
        condition.insert("id".to_string(), id);

        let res = self.storage.find_data_with_condition(&condition)?;

        let res = match res {
            None => return Err(AppError::new_entity_not_found(ENTITY_NAME, None).into()),
            Some(x) => x,
        };

        if res.base.status == 0 {
            return Err(AppError::new_entity_deleted(ENTITY_NAME, None).into());
        }

        let mut rest = RestaurantUpdate::default();
        rest.status = 0;

        return match self.storage.delete(&condition, &rest) {
            Ok(_) => Ok(()),
            Err(error) => Err(AppError::new_entity_deleted(ENTITY_NAME, error).into()),
        };
    }
}

pub fn new_delete_restaurant_business(
    storage: Arc<dyn DeleteRestaurantStorage>,
) -> Arc<dyn DeleteRestaurantBusiness> {
    return Arc::new(DefaultDeleteRestaurantBusiness::new(storage));
}
