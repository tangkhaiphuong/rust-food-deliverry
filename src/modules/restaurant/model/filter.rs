use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct Filter {
    #[serde(default)]
    pub user_id: Option<String>,
}

impl Display for Filter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}
