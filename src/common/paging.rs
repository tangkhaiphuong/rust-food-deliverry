#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct Paging {
    #[serde(default)]
    pub page: i32,
    #[serde(default)]
    pub limit: i32,
    #[serde(default)]
    pub total: i32,
}

impl Paging {
    pub fn new(page: i32, limit: i32, total: i32) -> Self {
        return Self { page, limit, total };
    }
    pub fn normalize(mut self) -> Self {
        if self.page <= 0 {
            self.page = 1
        }

        if self.limit <= 0 || self.limit > 100 {
            self.limit = 10
        }

        return self;
    }
}
