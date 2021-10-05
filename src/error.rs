/*
//  Copyright 2021 neko.rs contributors <https://neko.rs>
//
//  Licensed with GNU Affero General Public License v3.0 or later
*/

use actix_http::http::StatusCode;
use actix_web::{error::ResponseError, Error as ActixError, HttpResponse};
use askama::Error as AskamaError;
use diesel::result::Error as DieselError;
use http::header::InvalidHeaderValue;
use oauth2::reqwest::Error as ReqwestError;
use oauth2::{ErrorResponse, RequestTokenError};
use serde_json::Error as SerdeError;
use std::{
  error::Error,
  fmt,
  io::{Error as IOError, ErrorKind},
};
use url::ParseError as UrlParseError;

#[derive(Debug)]
pub enum WebError {
  NotFound,
  Unauthorized,
  ServerError(String, String),
}

impl fmt::Display for WebError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      WebError::NotFound => write!(f, "NotFound"),
      WebError::Unauthorized => write!(f, "Unauthorized"),
      WebError::ServerError(name, error) => write!(f, "{}: {}", name, error),
    }
  }
}

impl ResponseError for WebError {
  fn error_response(&self) -> HttpResponse {
    HttpResponse::BadRequest()
      .status(match self {
        WebError::NotFound => StatusCode::NOT_FOUND,
        WebError::Unauthorized => StatusCode::UNAUTHORIZED,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
      })
      .finish()
  }
}

impl From<ActixError> for WebError {
  fn from(err: ActixError) -> WebError {
    WebError::ServerError("ActixError".into(), err.to_string())
  }
}

impl From<DieselError> for WebError {
  fn from(err: DieselError) -> WebError {
    match err {
      DieselError::NotFound => WebError::NotFound,
      _ => WebError::ServerError("DieselError".into(), err.to_string()),
    }
  }
}

impl From<AskamaError> for WebError {
  fn from(err: AskamaError) -> WebError {
    WebError::ServerError("AskamaError".into(), err.to_string())
  }
}

impl From<IOError> for WebError {
  fn from(err: IOError) -> WebError {
    match err.kind() {
      ErrorKind::NotFound => WebError::NotFound,
      _ => WebError::ServerError("IOError".into(), err.to_string()),
    }
  }
}

impl<E: Error, ER: ErrorResponse> From<RequestTokenError<E, ER>> for WebError {
  fn from(err: RequestTokenError<E, ER>) -> WebError {
    WebError::ServerError("RequestTokenError".into(), err.to_string())
  }
}

impl From<SerdeError> for WebError {
  fn from(err: SerdeError) -> WebError {
    WebError::ServerError("SerdeError".into(), err.to_string())
  }
}

impl From<InvalidHeaderValue> for WebError {
  fn from(err: InvalidHeaderValue) -> WebError {
    WebError::ServerError("InvalidHeaderValue".into(), err.to_string())
  }
}

impl From<UrlParseError> for WebError {
  fn from(err: UrlParseError) -> WebError {
    WebError::ServerError("UrlParseError".into(), err.to_string())
  }
}

impl<T> From<ReqwestError<T>> for WebError
where
  T: Error,
{
  fn from(err: ReqwestError<T>) -> WebError {
    WebError::ServerError("ReqwestError".into(), err.to_string())
  }
}
