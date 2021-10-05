/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{error::WebError, models::memes::Meme};

use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, HttpResponse, Scope};
use http::{header, HeaderValue};
use std::path::{Path, PathBuf};

#[get("/{meme:[a-z0-9]+}/{res:[0-4]}.webm")]
async fn get_webm(
  req: HttpRequest, web::Path((name, res)): web::Path<(String, usize)>,
) -> Result<HttpResponse, WebError> {
  let res_str = ["144", "360", "720"];
  let lowest = get(&name, res_str[0]);
  if lowest.exists() {
    Meme::add_view((&name).to_owned())?;
    for r in (1..=res).rev() {
      let file = get(&name, res_str[r]);
      if file.exists() {
        return Ok(open(req, file)?);
      }
    }
    return Ok(open(req, lowest)?);
  } else {
    return Err(WebError::NotFound);
  }
}

fn get(name: &str, quality: &str) -> PathBuf {
  Path::new("./media/").join(quality.to_string()).join(name).with_extension("webm")
}

fn open(req: HttpRequest, path: PathBuf) -> Result<HttpResponse, WebError> {
  let mut res = NamedFile::open(path)?.into_response(&req)?;
  let headers = res.headers_mut();
  headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("max-age=7776000"));
  Ok(res)
}

#[get("/{meme:[a-z0-9]+}/thumb.webp")]
async fn webp_thumbnail(
  req: HttpRequest, web::Path(meme): web::Path<String>,
) -> Result<HttpResponse, WebError> {
  Ok(open(req, Path::new("./media/thumbs/").join(meme).with_extension("webp"))?)
}

pub fn scope() -> Scope {
  web::scope("/m").service(get_webm).service(webp_thumbnail)
}
