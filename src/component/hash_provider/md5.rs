use std::fmt::{Display, Formatter};

use md5::compute;

use crate::component::hash_provider::HashProvider;

#[derive(Debug, Default)]
pub struct Md5 {}

impl Md5 {}

impl HashProvider for Md5 {
    fn hash(&self, data: &str) -> String {
        let digest = compute(data);

        let result = format!("{:x}", digest);

        return result;
    }
}

impl Display for Md5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

pub fn new_hash_provider() -> Box<dyn HashProvider> {
    return Box::new(Md5::default());
}
