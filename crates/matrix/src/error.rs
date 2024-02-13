use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MatrixError {
    pub errcode: String,
    pub error: String,
}
