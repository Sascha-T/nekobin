/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{
  error::WebError,
  models::{memes::Meme, users::User, ApiQuery},
};
use actix_web::{
  get,
  web::{self, Path, Query},
  HttpResponse, Scope,
};

#[get("/{id:[0-9]{15,19}}")]
async fn get_user(Path(id): Path<i64>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().json(User::from_id(id)?))
}

#[get("/{id:[0-9]{15,19}}/memes")]
async fn get_user_memes(Path(id): Path<i64>, q: Query<ApiQuery>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().json(Meme::from_uid(id, q.into_inner())?))
}

pub fn scope() -> Scope {
  web::scope("/users").service(get_user).service(get_user_memes)
}
