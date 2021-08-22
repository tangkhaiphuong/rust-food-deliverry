#![allow(dead_code)]
// #![allow(unused_imports)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use std::{env, io};

use actix_web::web::Data;
use actix_web::{middleware, web, App, HttpServer};

use crate::component::{AppContext, DefaultAppContext, EnvSetting};
use crate::modules::restaurant::transport::web_actix::*;

mod common;
mod component;
mod modules;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    let setting = EnvSetting::new_from_environment();
    HttpServer::new(move || {
        let app_context: Box<dyn AppContext> = Box::new(DefaultAppContext::new(setting.clone()));

        let v1 = web::scope("/v1/restaurants")
            .service(
                web::resource("{id}")
                    .route(web::get().to(get_restaurant))
                    .route(web::delete().to(delete_restaurant))
                    .route(web::put().to(update_restaurant)),
            )
            .service(
                web::resource("")
                    .route(web::post().to(create_restaurant))
                    .route(web::get().to(list_restaurant)),
            );

        App::new()
            .app_data(Data::new(app_context))
            .wrap(middleware::Logger::default())
            .service(v1)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
