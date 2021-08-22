use crate::common::AppError;
use crate::modules::restaurant::model::RestaurantCreate;
use crate::modules::restaurant::model::ENTITY_NAME;
use std::error::Error;
use std::sync::Arc;

pub trait CreateRestaurantStorage {
    fn create(&self, data: &RestaurantCreate) -> Result<i32, Box<dyn Error + Send + Sync>>;
}

pub trait CreateNewRestaurantBusiness {
    fn create_restaurant(
        &self,
        data: &RestaurantCreate,
    ) -> Result<i32, Box<dyn Error + Send + Sync>>;
}

struct DefaultCreateNewRestaurantBusiness {
    storage: Arc<dyn CreateRestaurantStorage>,
}

impl DefaultCreateNewRestaurantBusiness {
    fn new(storage: Arc<dyn CreateRestaurantStorage>) -> Self {
        return DefaultCreateNewRestaurantBusiness { storage };
    }
}

impl CreateNewRestaurantBusiness for DefaultCreateNewRestaurantBusiness {
    fn create_restaurant(
        &self,
        data: &RestaurantCreate,
    ) -> Result<i32, Box<dyn Error + Send + Sync>> {
        return match self.storage.create(data) {
            Ok(value) => Ok(value),
            Err(error) => {
                return Err(AppError::new_cannot_create_entity(ENTITY_NAME, error).into());
            }
        };
    }
}

pub fn new_create_new_restaurant_business(
    storage: Arc<dyn CreateRestaurantStorage>,
) -> Arc<dyn CreateNewRestaurantBusiness> {
    return Arc::new(DefaultCreateNewRestaurantBusiness::new(storage));
}
