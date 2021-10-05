/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use lazy_static::lazy_static;
use std::env;

#[rustfmt::skip]
lazy_static! {
  pub static ref PORT: String = env::var("PORT").expect("PORT");
  pub static ref DOMAIN: String = env::var("DOMAIN").expect("DOMAIN");
  pub static ref CLIENT_ID: String = env::var("CLIENT_ID").expect("CLIENT_ID");
  pub static ref CLIENT_SECRET: String = env::var("CLIENT_SECRET").expect("CLIENT_SECRET");
  pub static ref COOKIE_SECRET: String = env::var("COOKIE_SECRET").expect("COOKIE_SECRET");
  pub static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL");
  pub static ref DEV_ENV: bool = env::var("DEV_ENV").is_ok() && env::var("DEV_ENV").unwrap().to_uppercase() == "TRUE";
}
