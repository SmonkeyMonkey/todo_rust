extern crate openssl;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

mod counter;
mod database;
mod json_serilization;
mod jwt;
mod models;
mod schema;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    const ALLOWED_VERSION: &'static str = "v1";

    let site_counter = counter::Counter { count: 0 };
    site_counter.save();

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_origin()
            .allow_any_method();
        App::new()
            .wrap_fn(|req, srv| {
                println!("{}", ALLOWED_VERSION);
                let passed: bool;
                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                println!("count {:?}", &site_counter);
                site_counter.save();
                if *&req.path().contains(&format!("/{}/", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
                }
                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => {
                        let resp = HttpResponse::NotImplemented()
                            .body(format!("only {} API is suported", ALLOWED_VERSION));
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::view_factory)
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
