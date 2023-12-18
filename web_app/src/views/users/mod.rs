use actix_web::web::{post, scope, ServiceConfig};

pub mod create;

pub fn user_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/user").route("create", post().to(create::create)));
}
