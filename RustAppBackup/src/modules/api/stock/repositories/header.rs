#[derive(Debug, Deserialize, IntoParams, Serialize, ToSchema)]
#[into_params(parameter_in = Header)]
#[schema(example = "Bearer")]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    /// Authorization token header
    #[param(example = "Bearer 12345")]
    pub authorization: String,
}