/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_web::{web, Scope};

mod memes;
mod users;

pub fn scope() -> Scope {
  web::scope("/api").service(memes::scope()).service(users::scope())
}
