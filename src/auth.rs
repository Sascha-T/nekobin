/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{config, error::WebError, models::users::UpdateUser};
use actix_http::{http::Cookie, HttpMessage};
use actix_session::Session;
use actix_web::{
  get,
  http::header,
  web::{self, Data, Query},
  HttpRequest, HttpResponse,
};
use http::{HeaderMap, HeaderValue, Method};
use oauth2::{
  basic::BasicClient, reqwest::http_client, AccessToken, AuthorizationCode, CsrfToken, Scope,
  TokenResponse,
};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct AuthInfo {
  user: UpdateUser,
}

#[derive(Deserialize)]
struct AuthRequest {
  code: String,
  state: String,
}

#[get("/login")]
async fn login(data: web::Data<BasicClient>) -> HttpResponse {
  let (url, state) =
    &data.authorize_url(CsrfToken::new_random).add_scope(Scope::new("identify".into())).url();

  HttpResponse::Found()
    .header(header::LOCATION, url.to_string())
    .cookie(
      Cookie::build("state", state.secret())
        .domain((*config::DOMAIN).to_owned())
        .path("/")
        .secure(!*config::DEV_ENV)
        .http_only(true)
        .finish(),
    )
    .finish()
}

#[get("/logout")]
async fn logout(session: Session) -> HttpResponse {
  session.purge();
  HttpResponse::Found().header(header::LOCATION, "/".to_string()).finish()
}

#[get("/callback")]
async fn callback(
  req: HttpRequest, session: Session, client: Data<BasicClient>, auth: Query<AuthRequest>,
) -> Result<HttpResponse, WebError> {
  // Verify that the state returned is the same as the cookie
  let state = auth.state.clone();
  let cookie = req.cookie("state");
  if cookie.is_none() || state != cookie.unwrap().value() {
    return Err(WebError::Unauthorized);
  }

  let code = AuthorizationCode::new(auth.code.clone());
  let token = &client.exchange_code(code).request(http_client)?;
  let user = req_user(token.access_token())?.user;
  user.update()?;
  session.set("uid", user.id)?;
  Ok(HttpResponse::Found().header(header::LOCATION, "/".to_string()).finish())
}

fn req_user(token: &AccessToken) -> Result<AuthInfo, WebError> {
  let mut headers = HeaderMap::new();
  headers
    .insert("Authorization", HeaderValue::from_str(format!("Bearer {}", token.secret()).as_str())?);
  let resp = http_client(oauth2::HttpRequest {
    url: Url::parse("https://discord.com/api/oauth2/@me")?,
    method: Method::GET,
    headers,
    body: Vec::new(),
  })?;
  Ok(serde_json::from_slice(&resp.body)?)
}

#[rustfmt::skip]
pub fn scope() -> actix_web::Scope {
  web::scope("/auth")
    .service(login)
    .service(logout)
    .service(callback)
}
