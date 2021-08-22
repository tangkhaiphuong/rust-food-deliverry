use std::env;

const CONNECTION_STRING: &str = "CONNECTION_STRING";
const SECRET_KEY: &str = "SECRET_KEY";
const S3_BUCKET: &str = "S3_BUCKET";
const S3_REGION: &str = "S3_REGION";
const S3_API_KEY: &str = "S3_API_KEY";
const S3_SECRET_KEY: &str = "S3_SECRET_KEY";
const S3_DOMAIN: &str = "S3_DOMAIN";

#[derive(Clone, Default, Debug)]
pub struct EnvSetting {
    pub aws_s3_setting: AwsS3Setting,
    pub secret_key: String,
    pub connection_string: String,
}

impl EnvSetting {
    pub fn new_from_environment() -> Self {
        let connection_string = env::var(CONNECTION_STRING).unwrap_or_default(); //.expect("DB_CONN_STR not found");
        let secret_key = env::var("SECRET_KEY").unwrap_or_default();
        return Self {
            connection_string,
            secret_key,
            aws_s3_setting: AwsS3Setting::new_from_environment(),
        };
    }
}

#[derive(Clone, Default, Debug)]
pub struct AwsS3Setting {
    bucket: String,
    region: String,
    api_key: String,
    secret_key: String,
    domain: String,
}

impl AwsS3Setting {
    fn new_from_environment() -> Self {
        let bucket = env::var(S3_BUCKET).unwrap_or_default();
        let region = env::var(S3_REGION).unwrap_or_default();
        let api_key = env::var(S3_API_KEY).unwrap_or_default();
        let secret_key = env::var(S3_SECRET_KEY).unwrap_or_default();
        let domain = env::var(S3_DOMAIN).unwrap_or_default();
        return Self {
            bucket,
            region,
            api_key,
            secret_key,
            domain,
        };
    }
}
