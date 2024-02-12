#[derive(Clone, Debug, Deserialize)]
pub struct MatrixError {
    pub error_code: String,
    pub error: String,
}
