use std::sync::Arc;

use crate::common::Image;
use crate::component::UploadProvider;
use std::error::Error;

pub trait UpdateImageBusiness {
    fn save_file_uploaded(
        &self,
        data: &[u8],
        destination: String,
    ) -> Result<Image, Box<dyn Error + Send + Sync>>;
}

struct DefaultUpdateImageBusiness {
    upload_provider: Arc<dyn UploadProvider>,
}

impl DefaultUpdateImageBusiness {
    fn new(upload_provider: Arc<dyn UploadProvider>) -> Self {
        return DefaultUpdateImageBusiness { upload_provider };
    }
}

impl UpdateImageBusiness for DefaultUpdateImageBusiness {
    fn save_file_uploaded(
        &self,
        _data: &[u8],
        _destination: String,
    ) -> Result<Image, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}

pub fn new_update_image_business(
    upload_provider: Arc<dyn UploadProvider>,
) -> Arc<dyn UpdateImageBusiness> {
    return Arc::new(DefaultUpdateImageBusiness::new(upload_provider));
}
