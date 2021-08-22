use std::convert::TryFrom;
use std::error::Error;

use crate::modules::restaurant::model::{RestaurantCreate, RestaurantUpdate};

table! {
    restaurants (id) {
        id -> Nullable<Integer>,
        status -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        name -> Nullable<VarChar>,
        addr -> Nullable<VarChar>,
        owner_id -> Nullable<Integer>,
        city_id -> Nullable<Integer>,
        logo -> Nullable<VarChar>,
        cover -> Nullable<VarChar>,
    }
}

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[table_name = "restaurants"]
pub struct RestaurantEntity {
    pub id: Option<i32>,
    pub status: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub name: Option<String>,
    pub addr: Option<String>,
    pub owner_id: Option<i32>,
    pub city_id: Option<i32>,
    pub logo: Option<String>,
    pub cover: Option<String>,
}

impl TryFrom<RestaurantCreate> for RestaurantEntity {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(model: RestaurantCreate) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: None,
            status: model.base.status.into(),
            created_at: None,
            updated_at: None,
            name: model.name.into(),
            addr: model.address.into(),
            owner_id: model.owner_id.into(),
            city_id: model.city_id.into(),
            logo: serde_json::to_string(&model.logo)
                .map_err(|e| Box::new(e))?
                .into(),
            cover: serde_json::to_string(&model.cover)
                .map_err(|e| Box::new(e))?
                .into(),
        });
    }
}

impl TryFrom<&RestaurantCreate> for RestaurantEntity {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(model: &RestaurantCreate) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: None,
            status: model.base.status.into(),
            created_at: None,
            updated_at: None,
            name: model.name.clone().into(),
            addr: model.address.clone().into(),
            owner_id: model.owner_id.into(),
            city_id: model.city_id.into(),
            logo: serde_json::to_string(&model.logo)
                .map_err(|e| Box::new(e))?
                .into(),
            cover: serde_json::to_string(&model.cover)
                .map_err(|e| Box::new(e))?
                .into(),
        });
    }
}

impl TryFrom<RestaurantUpdate> for RestaurantEntity {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(model: RestaurantUpdate) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: None,
            status: model.status.into(),
            created_at: None,
            updated_at: None,
            name: model.name.into(),
            addr: model.address.into(),
            owner_id: model.owner_id.into(),
            city_id: None,
            logo: serde_json::to_string(&model.logo)
                .map_err(|e| Box::new(e))?
                .into(),
            cover: serde_json::to_string(&model.cover)
                .map_err(|e| Box::new(e))?
                .into(),
        });
    }
}

impl TryFrom<&RestaurantUpdate> for RestaurantEntity {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(model: &RestaurantUpdate) -> Result<Self, Self::Error> {
        return Ok(Self {
            id: None,
            status: model.status.into(),
            created_at: None,
            updated_at: None,
            name: model.name.clone().into(),
            addr: model.address.clone().into(),
            owner_id: model.owner_id.into(),
            city_id: None,
            logo: serde_json::to_string(&model.logo)
                .map_err(|e| Box::new(e))?
                .into(),
            cover: serde_json::to_string(&model.cover)
                .map_err(|e| Box::new(e))?
                .into(),
        });
    }
}
