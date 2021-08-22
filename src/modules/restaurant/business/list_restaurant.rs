use crate::common::AppError;
use crate::common::Paging;
use crate::modules::restaurant::model::Filter;
use crate::modules::restaurant::model::{Restaurant, ENTITY_NAME};
use std::error::Error;
use std::sync::Arc;

pub trait ListRestaurantStorage {
    fn list_data_with_condition(
        &self,
        filter: &Filter,
        paging: &Paging,
    ) -> Result<Vec<Restaurant>, Box<dyn Error + Send + Sync>>;
}

pub trait ListRestaurantBusiness {
    fn list_restaurant(
        &self,
        filter: &Filter,
        paging: &Paging,
    ) -> Result<Vec<Restaurant>, Box<dyn Error + Send + Sync>>;
}

pub struct DefaultListRestaurantBusiness {
    pub storage: Arc<dyn ListRestaurantStorage>,
}

impl DefaultListRestaurantBusiness {
    pub fn new(storage: Arc<dyn ListRestaurantStorage>) -> Self {
        return DefaultListRestaurantBusiness { storage };
    }
}

impl ListRestaurantBusiness for DefaultListRestaurantBusiness {
    fn list_restaurant(
        &self,
        filter: &Filter,
        paging: &Paging,
    ) -> Result<Vec<Restaurant>, Box<dyn Error + Send + Sync>> {
        return match self.storage.list_data_with_condition(filter, paging) {
            Ok(value) => Ok(value),
            Err(error) => return Err(AppError::new_cannot_list_entity(ENTITY_NAME, error).into()),
        };
    }
}

pub fn new_list_restaurant_business(
    storage: Arc<dyn ListRestaurantStorage>,
) -> Arc<dyn ListRestaurantBusiness> {
    return Arc::new(DefaultListRestaurantBusiness::new(storage));
}
