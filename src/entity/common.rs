use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Resp<T> {
    pub(crate) data: Option<T>,
    pub(crate) message: Option<String>,
}