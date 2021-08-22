use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Formatter};

use chrono::{Local, TimeZone};

use crate::common::{Image, SqlModel};
use crate::modules::restaurant::model::schema::RestaurantEntity;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct Restaurant {
    #[serde(flatten)]
    pub base: SqlModel,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub owner_id: i32,
    #[serde(default)]
    pub city_id: i32,
    #[serde(default)]
    pub logo: Image,
    #[serde(default)]
    pub cover: Vec<Image>,
}

impl Display for Restaurant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

impl TryFrom<RestaurantEntity> for Restaurant {
    type Error = Box<dyn Error + Send + Sync>;

    fn try_from(entity: RestaurantEntity) -> Result<Self, Self::Error> {
        return Ok(Self {
            base: SqlModel {
                id: entity.id.ok_or("Cannot get id")?,
                fake_id: Default::default(),
                status: entity.status.ok_or("Cannot get status")?,
                created_at: Local
                    .from_local_datetime(&entity.created_at.ok_or("Cannot get created at")?)
                    .single()
                    .ok_or("Cannot parse created at")?
                    .into(),

                updated_at: Local
                    .from_local_datetime(&entity.updated_at.ok_or("Cannot get updated at")?)
                    .single()
                    .ok_or("Cannot convert updated at")?
                    .into(),
            },
            name: entity.name.ok_or("Cannot get name")?,
            address: entity.addr.ok_or("Cannot get address")?,
            owner_id: entity.owner_id.ok_or("Cannot get owner id")?,
            city_id: entity.city_id.ok_or("Cannot get city id")?,
            logo: serde_json::from_str(&entity.logo.ok_or("Cannot get logo")?)
                .map_err(|e| Box::new(e))?,
            cover: serde_json::from_str(&entity.cover.ok_or("Cannot get cover")?)
                .map_err(|e| Box::new(e))?,
        });
    }
}
