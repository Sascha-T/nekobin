/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{
  error::WebError,
  models::{memes::MemeUser, ApiQuery},
};
use actix_web::{
  get,
  web::{self, Path, Query},
  HttpResponse, Scope,
};

#[get("")]
async fn get_memes(q: Query<ApiQuery>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().json(MemeUser::all(q.into_inner())?))
}

#[get("/{name:[a-z0-9]{1,24}}")]
async fn get_meme(Path(name): Path<String>) -> Result<HttpResponse, WebError> {
  Ok(HttpResponse::Ok().json(MemeUser::from_name(name)?))
}

pub fn scope() -> Scope {
  web::scope("/memes").service(get_memes).service(get_meme)
}
