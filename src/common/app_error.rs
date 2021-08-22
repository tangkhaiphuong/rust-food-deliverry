use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct AppError {
    #[serde(default)]
    pub status_code: u16,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub log: String,
    #[serde(default)]
    pub log_key: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub root_error: Option<Box<dyn Error + Send + Sync>>,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

impl Error for AppError {}

impl AppError {
    pub fn new<
        T: Into<String>,
        K: Into<String>,
        P: Into<String>,
        N: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        status_code: u16,
        message: T,
        log: K,
        log_key: P,
        error: N,
    ) -> Self {
        return AppError {
            status_code,
            message: message.into(),
            log: log.into(),
            log_key: log_key.into(),
            root_error: error.into(),
        };
    }

    pub fn new_bad_request<
        T: Into<String>,
        K: Into<String>,
        P: Into<String>,
        N: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        message: T,
        log: K,
        log_key: P,
        error: N,
    ) -> Self {
        return AppError {
            status_code: 400,
            message: message.into(),
            log: log.into(),
            log_key: log_key.into(),
            root_error: error.into(),
        };
    }

    pub fn new_unauthorized<
        T: Into<String>,
        K: Into<String>,
        P: Into<String>,
        N: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        message: T,
        log: K,
        log_key: P,
        error: N,
    ) -> Self {
        return AppError {
            status_code: 401,
            message: message.into(),
            log: log.into(),
            log_key: log_key.into(),
            root_error: error.into(),
        };
    }

    pub fn new_custom_error<
        T: Into<String> + Clone,
        K: Into<String> + Clone,
        P: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        message: T,
        log_key: K,
        error: P,
    ) -> Self {
        let temp = error.into();
        return match &temp {
            Some(error_inner) => {
                return AppError::new_bad_request(message, error_inner.to_string(), log_key, temp);
            }
            _ => AppError::new_bad_request(
                message.clone(),
                message.clone(),
                log_key,
                Some(message.clone().into().into()),
            ),
        };
    }

    pub fn root_error(&self) -> Option<Box<dyn Error + Send + Sync>> {
        let error = match self.root_error {
            None => return None,
            Some(ref error) => error,
        };

        let parent: Option<&AppError> = error.downcast_ref();

        return match parent {
            None => None,
            Some(app_error) => app_error.root_error(),
        };
    }

    pub fn new_error_db<T: Into<Box<dyn Error + Send + Sync>>>(error: T) -> Self {
        let error = error.into();
        return Self::new(
            500,
            "something went wrong with DB".to_string(),
            error.to_string(),
            "DB_ERROR".to_string(),
            Some(error),
        );
    }

    pub fn new_invalid_request(error: Box<dyn Error + Send + Sync>) -> Self {
        return Self::new_bad_request(
            "invalid request".to_string(),
            error.to_string(),
            "ErrInvalidRequest".to_string(),
            Some(error),
        );
    }

    pub fn new_internal(error: Box<dyn Error + Send + Sync>) -> Self {
        return Self::new(
            500,
            "something went wrong in the server".to_string(),
            error.to_string(),
            "ErrInternal".to_string(),
            Some(error),
        );
    }

    pub fn new_cannot_list_entity<
        T: Into<String>,
        K: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        return Self::new_custom_error(
            format!("Cannot list {}", entity.to_lowercase()),
            format!("ErrCannotList{}", entity),
            error,
        );
    }

    pub fn new_cannot_delete_entity<
        T: Into<String>,
        K: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        return Self::new_custom_error(
            format!("Cannot delete {}", entity.to_lowercase()),
            format!("ErrCannotDelete{}", entity),
            error,
        );
    }

    pub fn new_cannot_update_entity<
        T: Into<String>,
        K: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        return Self::new_custom_error(
            format!("Cannot upload {}", entity.to_lowercase()),
            format!("ErrCannotUpdate{}", entity),
            error,
        );
    }

    pub fn new_cannot_get_entity<T: Into<String>, K: Into<Option<Box<dyn Error + Send + Sync>>>>(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        return Self::new_custom_error(
            format!("Cannot get {}", entity.to_lowercase()),
            format!("ErrCannotGet{}", entity),
            error,
        );
    }

    pub fn new_entity_deleted<T: Into<String>, K: Into<Option<Box<dyn Error + Send + Sync>>>>(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        let mut c = entity.chars();
        let name = match c.next() {
            None => String::default(),
            Some(f) => f
                .to_uppercase()
                .chain(c.flat_map(|t| t.to_lowercase()))
                .collect(),
        };

        return Self::new_custom_error(
            format!("{} deleted", name),
            format!("Err{}Deleted", entity),
            error,
        );
    }

    pub fn new_entity_existed<T: Into<String>, K: Into<Option<Box<dyn Error + Send + Sync>>>>(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        return Self::new_custom_error(
            format!("{} already exists", entity.to_lowercase()),
            format!("Err{}sAlreadyExists", entity),
            error,
        );
    }

    pub fn new_entity_not_found<T: Into<String>, K: Into<Option<Box<dyn Error + Send + Sync>>>>(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        return Self::new_custom_error(
            format!("{} not found", entity.to_lowercase()),
            format!("Err{}sNotFound", entity),
            error,
        );
    }

    pub fn new_cannot_create_entity<
        T: Into<String>,
        K: Into<Option<Box<dyn Error + Send + Sync>>>,
    >(
        entity: T,
        error: K,
    ) -> Self {
        let entity = entity.into();
        return Self::new_custom_error(
            format!("Cannot create {}", entity.to_lowercase()),
            format!("ErrCannotCreate{}", entity),
            error.into(),
        );
    }

    pub fn new_no_permission(error: Option<Box<dyn Error + Send + Sync>>) -> Self {
        return Self::new_custom_error(
            "You have no permission".to_string(),
            "ErrNoPermission".to_string(),
            error,
        );
    }
}
