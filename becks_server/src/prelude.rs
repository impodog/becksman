pub(crate) use actix_web::{get, http, post, put, web, App, HttpResponse, HttpServer, Responder};
pub(crate) use becks_crew::check;
pub(crate) use log::{error, info, warn};
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use std::sync::Arc;

pub type DbData = web::Data<Arc<becks_db::Db>>;
