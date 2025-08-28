mod types;

use actix_web::{get, web, Responder, Scope};

pub fn module_scope() -> Scope {
    web::scope("/module").service(_get_module_root)
}

#[get("")]
async fn _get_module_root() -> impl Responder {
    "Module root endpoint"
}
