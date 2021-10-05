/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate serde_json;

use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{middleware, App, HttpServer};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use std::io;

pub mod config;
pub mod db;
pub mod error;
pub mod models;

mod api;
mod auth;
mod frontend;
mod media;

#[actix_web::main]
async fn main() -> io::Result<()> {
  if *config::DEV_ENV {
    println!("Running in a dev environment")
  }
  env_logger::init();
  HttpServer::new(|| {
    App::new()
      .data(
        BasicClient::new(
          ClientId::new((*config::CLIENT_ID).to_owned()),
          Some(ClientSecret::new((*config::CLIENT_SECRET).to_owned())),
          AuthUrl::new("https://discord.com/api/oauth2/authorize".into()).unwrap(),
          Some(TokenUrl::new("https://discord.com/api/oauth2/token".into()).unwrap()),
        )
        //TODO: Cleanup maybe
        .set_redirect_uri(
          RedirectUrl::new(format!(
            "{}://{}{}/auth/callback",
            if *config::DEV_ENV { "http" } else { "https" },
            *config::DOMAIN,
            if *config::DEV_ENV {
              format!(":{}", *config::PORT)
            } else {
              "".to_string()
            },
          ))
          .expect("Invalid redirect URL"),
        ),
      )
      .wrap(middleware::Logger::default())
      .wrap(
        CookieSession::signed((*config::COOKIE_SECRET).as_bytes())
          .domain((*config::DOMAIN).to_owned())
          .secure(!*config::DEV_ENV)
          .name("session")
          .http_only(true),
      )
      .service(media::scope())
      .service(Files::new("/assets", "./assets"))
      .service(api::scope())
      .service(auth::scope())
      .service(frontend::scope())
  })
  .bind(format!("0.0.0.0:{}", *config::PORT))?
  .run()
  .await
}
