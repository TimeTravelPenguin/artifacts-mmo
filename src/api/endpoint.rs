use reqwest::Method;
use serde::{Deserialize, Serialize};

pub trait APIEndpoint {
    type Request: Serialize;
    type Response: for<'de> Deserialize<'de>;

    fn path() -> &'static str;
    fn method() -> Method;
}
