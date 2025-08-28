use actix_web::{get, post, web};

use crate::{
    commons::{CommonInfo, ResponseResult, ResponseType, Status},
    handles::server::{
        apache::apache_scope,
        bind::bind_scope,
        dhcp::dhcp_scope,
        ftp::ftp_scope,
        ldap::ldap_scope,
        mysql::mysql_scope,
        nginx::nginx_scope,
        samba::samba_scope,
        squid::squid_scope,
        ssh::ssh_scope,
        stall::{stall_request, stalledResponse, stalled_request, Pcs},
    },
};

mod apache;
mod bind;
mod dhcp;
mod ftp;
mod ldap;
mod mysql;
mod nginx;
mod samba;
mod squid;
mod ssh;
mod stall;

pub fn server_scope() -> actix_web::Scope {
    actix_web::web::scope("/server")
        .service(apache_scope())
        .service(bind_scope())
        .service(dhcp_scope())
        .service(ldap_scope())
        .service(mysql_scope())
        .service(nginx_scope())
        .service(ftp_scope())
        .service(samba_scope())
        .service(squid_scope())
        .service(ssh_scope())
        .service(installed)
        .service(noinstall)
        .service(install)
}

#[get("/installed")]
async fn installed(data: web::Json<stalled_request>) -> web::Json<stalledResponse> {
    let server_name = &data.server;
    // Example test data
    let pcs = Pcs {
        uuids: CommonInfo {
            hostname: format!("{}-install-host", server_name),
            status:   Status::Active,
            cpu:      0.0,
            memory:   0.0,
        },
    };
    let response = stalledResponse { pcs, length: 0 };
    web::Json(response)
}

#[get("/noinstall")]
async fn noinstall(data: web::Json<stalled_request>) -> web::Json<stalledResponse> {
    let server_name = &data.server;
    // Example test data
    let pcs = Pcs {
        uuids: CommonInfo {
            hostname: format!("{}-noinstall-host", server_name),
            status:   Status::Stopped,
            cpu:      0.0,
            memory:   0.0,
        },
    };
    let response = stalledResponse { pcs, length: 0 };
    web::Json(response)
}

#[post("/install")]
async fn install(data: web::Json<stall_request>) -> web::Json<ResponseResult> {
    println!("{:#?}", data);
    web::Json(ResponseResult {
        r#type:  ResponseType::Ok,
        message: "install successful".to_string(),
    })
}
