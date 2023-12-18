use actix_web::web::{get, post, scope, ServiceConfig};

mod login;
mod logout;

pub fn auth_view_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .route("login", get().to(login::login))
            .route("login", post().to(login::login))
            .route("logout", get().to(logout::logout)),
    );
}
