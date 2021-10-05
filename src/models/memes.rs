/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use crate::{
  db::conn,
  models::{
    users::{users, User},
    ApiQuery, Sortable,
  },
  paginate, sort, sortable,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, result::Error, update};
use serde::Serialize;

table! {
  memes (id) {
    id -> Integer,
    name -> VarChar,
    user_id -> BigInt,
    timestamp -> BigInt,
    views -> BigInt,
    votes -> BigInt,
    resolution -> SmallInt,
    explicit -> Bool,
  }
}

joinable!(memes -> users (user_id));
sortable!(memes::table -> BigInt, (
  "timestamp" => memes::timestamp,
  "views" => memes::views,
  "votes" => memes::votes
));

#[derive(Queryable, Serialize)]
pub struct Meme {
  pub id: i32,
  pub name: String,
  pub user_id: i64,
  pub timestamp: i64,
  pub views: i64,
  pub votes: i64,
  pub resolution: i16,
  pub explicit: bool,
}

impl Meme {
  pub fn from_uid(uid: i64, q: ApiQuery) -> Result<Vec<Self>, Error> {
    paginate!(sort!(memes, &q).filter(memes::user_id.eq(uid)), &q, 100).load::<Self>(&conn())
  }

  pub fn add_view(name: String) -> Result<usize, Error> {
    update(memes::table.filter(memes::name.eq(name))).set(memes::views.eq(memes::views + 1)).execute(&conn())
  }
}

#[derive(Queryable, Serialize)]
pub struct MemeUser {
  pub meme: Meme,
  pub user: User,
}

impl MemeUser {
  pub fn from_name(name: String) -> Result<Self, Error> {
    memes::table.filter(memes::name.eq(name)).inner_join(users::table).first::<Self>(&conn())
  }

  pub fn all(q: ApiQuery) -> Result<Vec<Self>, Error> {
    paginate!(sort!(memes, &q).inner_join(users::table), &q, 100).load::<Self>(&conn())
  }
}
