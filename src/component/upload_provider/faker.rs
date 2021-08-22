use std::error::Error;

use crate::common::Image;
use crate::component::upload_provider::UploadProvider;

#[derive(Debug, Default)]
struct Faker {}

impl Faker {}

impl UploadProvider for Faker {
    fn save_file(&self, _data: &[u8], _dst: String) -> Result<Image, &dyn Error> {
        return Ok(Image {
            id: 0,
            url: "".to_string(),
            with: 0,
            height: 0,
            cloud_name: "".to_string(),
            extension: "".to_string(),
        });
    }

    fn get_domain(&self) -> String {
        todo!()
    }
}

pub fn new_faker() -> Box<dyn UploadProvider> {
    return Box::new(Faker::default());
}
