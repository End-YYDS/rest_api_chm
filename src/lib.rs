//#![allow(non_camel_case_types)]
//#![allow(dead_code)]
use crate::handles::handles_scope;
use actix_web::web::{scope, ServiceConfig};

mod commons;
mod handles;

pub fn configure_app(cfg: &mut ServiceConfig) {
    cfg.service(scope("/api").configure(handles_scope));
}
