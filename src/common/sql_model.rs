use std::fmt::{Display, Formatter};

use crate::common::uid::*;
use crate::common::Uid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct SqlModel {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: i32,
    #[serde(
        rename(serialize = "id", deserialize = "id"),
        skip_deserializing,
        with = "uid"
    )]
    pub fake_id: Uid,
    #[serde(default)]
    pub status: i32,
    pub created_at: Option<chrono::DateTime<chrono::Local>>,
    pub updated_at: Option<chrono::DateTime<chrono::Local>>,
}

impl SqlModel {
    pub fn mask(&mut self, object_type: u32, shard_id: u32) -> &mut Self {
        self.fake_id = Uid::new(self.id as u32, object_type, shard_id);
        return self;
    }
}

impl Display for SqlModel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

impl Default for SqlModel {
    fn default() -> Self {
        return Self {
            id: 0,
            fake_id: Default::default(),
            status: 0,
            created_at: Option::from(chrono::offset::Local::now()),
            updated_at: Option::from(chrono::offset::Local::now()),
        };
    }
}

//datetime_local
