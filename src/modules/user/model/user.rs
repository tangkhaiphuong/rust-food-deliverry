use crate::common::{Requester, SqlModel};

#[derive(Serialize, Deserialize, Debug, Default, Queryable)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct User {
    #[serde(flatten)]
    pub base: SqlModel,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub salt: String,
    #[serde(default)]
    pub last_name: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub avatar: String,
}

impl User {}

impl Requester for User {
    fn user_id(&self) -> i32 {
        return self.base.id;
    }

    fn email(&self) -> String {
        return self.email.clone();
    }

    fn role(&self) -> String {
        return self.role.clone();
    }
}
