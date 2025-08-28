use crate::handles::chm::{backup::backup_scope, mca::mca_scope, pc_manager::{pc_manager_scope, pcgroup_scope}};

pub mod backup;
pub mod mca;
pub mod pc_manager;

pub fn chm_scope() -> actix_web::Scope {
    actix_web::web::scope("/chm")
        .service(backup_scope())
        .service(mca_scope())
        .service(pc_manager_scope())
        .service(pcgroup_scope())
}
    