use serde::Serialize;
use crate::res;

pub trait Request: Serialize + std::fmt::Debug + Clone {
    type Response: res::Response;   
}