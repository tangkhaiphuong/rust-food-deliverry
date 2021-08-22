use std::fmt::Debug;
use std::fmt::{Display, Formatter};
use std::marker::PhantomData;

use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case" /* "camelCase" */)]
pub struct AppResponse<'a, 'b, 'c, T: Serialize + 'a, K: Serialize + 'b, P: Serialize + 'c> {
    #[serde(default)]
    data: T,
    #[serde(default)]
    paging: K,
    #[serde(default)]
    filter: P,
    #[serde(skip_serializing, skip_deserializing)]
    _1: PhantomData<&'a T>,
    _2: PhantomData<&'b K>,
    _3: PhantomData<&'c P>,
}

impl<'a, 'b, 'c, T: Serialize + 'a, K: Serialize + 'b, P: Serialize + 'c> Display
    for AppResponse<'a, 'b, 'c, T, K, P>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}

impl<'a, 'b, 'c, T: Serialize + 'a, K: Serialize + 'b, P: Serialize + 'c>
    AppResponse<'a, 'b, 'c, T, K, P>
{
    pub fn new(data: T, paging: K, filter: P) -> Self {
        return AppResponse {
            data,
            paging,
            filter,
            _1: PhantomData,
            _2: PhantomData,
            _3: PhantomData,
        };
    }
}

pub fn new_simple_success_response<'a, T: Serialize + 'a>(
    data: T,
) -> AppResponse<'a, 'static, 'static, T, (), ()> {
    return AppResponse {
        data,
        paging: (),
        filter: (),
        _1: PhantomData,
        _2: PhantomData,
        _3: PhantomData,
    };
}
