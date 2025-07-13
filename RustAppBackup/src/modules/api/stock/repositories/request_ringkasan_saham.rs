#[allow(dead_code)]
#[derive(ToSchema)]
#[schema(example = "Ringkasan Saham-20240530.xlsx")]
pub struct UploadRequestBody {
    #[schema(value_type = String, format = Binary)]
    file: String,
}