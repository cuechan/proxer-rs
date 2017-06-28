pub mod info;

use serde_json;


pub trait FromApi {
    fn from_api(data: serde_json::Value) -> Self;
}
