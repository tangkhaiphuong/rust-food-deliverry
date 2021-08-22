mod faker;

use crate::common::Image;
pub use faker::*;

pub trait UploadProvider {
    fn save_file(&self, data: &[u8], destination: String) -> Result<Image, &dyn std::error::Error>;
    fn get_domain(&self) -> String;
}
