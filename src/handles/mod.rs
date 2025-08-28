use crate::handles::{chm::chm_scope, login::login_scope, server::server_scope};
use actix_web::web::ServiceConfig;

pub mod chm;
pub mod login;
pub mod server;

pub fn handles_scope(cfg: &mut ServiceConfig) {
    cfg.service(login_scope()).service(server_scope()).service(chm_scope());
}
