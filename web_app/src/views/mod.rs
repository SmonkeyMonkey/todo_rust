use actix_web::web::ServiceConfig;

use self::{
    app::app_view_factory, auth::auth_view_factory, to_do::to_do_views_factory,
    users::user_views_factory,
};

mod app;
mod auth;
mod to_do;
mod users;

pub fn view_factory(app: &mut ServiceConfig) {
    auth_view_factory(app);
    to_do_views_factory(app);
    app_view_factory(app);
    user_views_factory(app);
}
