use crate::common::{Image, SqlModel};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Default, Queryable)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct RestaurantCreate {
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

impl Display for RestaurantCreate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}
