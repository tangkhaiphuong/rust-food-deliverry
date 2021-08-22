use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct Image {
    #[serde(default)]
    pub id: i32,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub with: i32,
    #[serde(default)]
    pub height: i32,
    #[serde(default)]
    pub cloud_name: String,
    #[serde(default)]
    pub extension: String,
}

impl Image {
    pub fn fulfill(&mut self, domain: String) -> &mut Self {
        self.url = format!("{}/{}", domain, self.url);
        return self;
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}
