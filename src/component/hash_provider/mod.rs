mod md5;

pub trait HashProvider {
    fn hash(&self, data: &str) -> String;
}
