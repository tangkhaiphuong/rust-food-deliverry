use std::any::Any;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::common::{AppError, Requester};
use crate::modules::restaurant::business::GetRestaurantStorage;
use crate::modules::restaurant::model::RestaurantUpdate;
use crate::modules::restaurant::model::ENTITY_NAME;

pub trait UpdateRestaurantStorage: GetRestaurantStorage {
    fn update(
        &self,
        condition: &HashMap<String, Arc<dyn Any + Send + Sync>>,
        data: &RestaurantUpdate,
    ) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

pub trait UpdateRestaurantBusiness {
    fn update_restaurant(
        &self,
        id: i32,
        data: &RestaurantUpdate,
    ) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

struct DefaultUpdateRestaurantBusiness {
    storage: Arc<dyn UpdateRestaurantStorage>,
    requester: Arc<dyn Requester>,
}

impl DefaultUpdateRestaurantBusiness {
    fn new(storage: Arc<dyn UpdateRestaurantStorage>, requester: Arc<dyn Requester>) -> Self {
        return DefaultUpdateRestaurantBusiness { storage, requester };
    }
}

impl UpdateRestaurantBusiness for DefaultUpdateRestaurantBusiness {
    fn update_restaurant(
        &self,
        id: i32,
        data: &RestaurantUpdate,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let mut condition = HashMap::new();
        let id: Arc<dyn Any + Send + Sync> = Arc::new(id);
        condition.insert("id".to_string(), id);

        let res = self
            .storage
            .find_data_with_condition(&condition)?
            .ok_or::<Box<dyn Error + Send + Sync>>(
                AppError::new_entity_not_found(ENTITY_NAME, None).into(),
            )?;

        if res.base.status == 0 {
            return Err(AppError::new_entity_deleted(ENTITY_NAME, None).into());
        }

        if self.requester.role() != "admin" && res.owner_id != self.requester.user_id() {
            return Err(AppError::new_no_permission(None).into());
        }

        let result = self
            .storage
            .update(&condition, data)
            .map_err(|e| Box::new(AppError::new_cannot_update_entity(ENTITY_NAME, e)))?;

        return Ok(result);
    }
}

pub fn new_update_restaurant_business(
    storage: Arc<dyn UpdateRestaurantStorage>,
    requester: Arc<dyn Requester>,
) -> Arc<dyn UpdateRestaurantBusiness> {
    return Arc::new(DefaultUpdateRestaurantBusiness::new(storage, requester));
}
