/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{error::WebError, models::users::User};
use actix_http::http::header::ContentType;
use actix_session::Session;
use actix_web::{HttpResponse, get, web::Path};
use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomePage {
  user: Option<User>,
  sort: String,
}

#[get("/{name:(hot|top|new)?}")]
async fn render(session: Session, Path(name): Path<String>) -> Result<HttpResponse, WebError> {
  Ok(
    HttpResponse::Ok().set(ContentType(mime::TEXT_HTML)).body(
      HomePage {
        user: User::from_session(session)?,
        sort: match name.as_str() {
          "hot" => "votes",
          "top" => "views",
          _ => "timestamp"
        }.to_owned()
      }
      .render()?,
    ),
  )
}
