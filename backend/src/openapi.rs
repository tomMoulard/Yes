use utoipa::OpenApi;

use crate::models::User;

#[derive(OpenApi)]
#[openapi(paths(), components(schemas(User)))]
pub struct ApiDoc;
