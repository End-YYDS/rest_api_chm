mod module;

use crate::handles::chm::setting::module::module_scope;
use actix_web::{web, Scope};

pub fn setting_scope() -> Scope {
    web::scope("/setting").service(module_scope())
}
