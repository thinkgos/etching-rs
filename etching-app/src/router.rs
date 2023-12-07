use actix_web::web;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify,
};

use crate::handler::dict;
use crate::handler::passport;
use crate::handler::public;

#[derive(utoipa::OpenApi)]
#[openapi(
    info(
        license(
            name = "Apache-2.0",
            url = "https://www.apache.org/licenses/LICENSE-2.0.txt",
        ),
    ),
    servers(
        (
            url = "/api", 
            description = "Develop Server",
        ),
        (
            url = "http://{host}:{port}/api", 
            description = "Remote Server",
            variables (
                ("host" = (default = "kata.thinkgos.cn", enum_values("kata.thinkgos.cn"), description = "Supported urls for API")),
                ("port" = (default = "9999", enum_values("9999", "9527"), description = "Supported ports for API"))
            )
        ),
    ),
    security(("Token" = []),("ApiKey" = [])),
    modifiers(&SecurityAddon),
    components(schemas(),responses()),
    paths(
        public::login,
        public::healthy,
        dict::list_dict,
        dict::get_dict,
    ),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .get_or_insert_with(|| utoipa::openapi::ComponentsBuilder::new().build());
        // Authorization: Bearer xxx
        components.add_security_scheme(
            "Token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        // Authorization: xxx
        components.add_security_scheme(
            "ApiKey",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
        );
    }
}

pub fn configure_api(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").configure(configure_v1))
        .configure(configure_v2);
}

fn configure_v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            // public
            .service(public::login)
            .service(public::healthy)
            // dict
            .service(dict::list_dict)
            .service(dict::get_dict)
            .configure(passport::config),
    );
}

fn configure_v2(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/v2"));
}
