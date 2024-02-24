use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MatrixError {
    #[serde(rename = "errcode")]
    pub error_code: String,
    #[serde(rename = "error")]
    pub error: String,
}
