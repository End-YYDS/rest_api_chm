use actix_web::{get, post, web, Scope};

use crate::{
    commons::{ResponseResult, ResponseType},
    handles::chm::pc_manager::types::{
        DeletePcGroupRequest, DeletePcRequest, GetPcgroupResponseResult, PCManagerRequest,
        PatchPcgroupRequest, PcInformation, Pcs, PostPcgroupRequest, PutPcgroupRequest,
        SpecificRequest, Uuid, UuidsRequest,
    },
};

pub mod types;

pub fn pc_manager_scope() -> Scope {
    web::scope("/pc")
        .route("", web::delete().to(delete_pc))
        .route("/", web::delete().to(delete_pc))
        .service(add)
        .service(all)
        .service(specific)
        .service(reboot)
        .service(shutdown)
}

pub fn pcgroup_scope() -> Scope {
    web::scope("/pcgroup")
        .route("", web::post().to(post_pcgroup))
        .route("/", web::post().to(post_pcgroup))
        .route("", web::get().to(get_pcgroup))
        .route("/", web::get().to(get_pcgroup))
        .route("", web::put().to(put_pcgroup))
        .route("/", web::put().to(put_pcgroup))
        .route("", web::patch().to(patch_pcgroup))
        .route("/", web::patch().to(patch_pcgroup))
        .route("", web::delete().to(delete_pcgroup))
        .route("/", web::delete().to(delete_pcgroup))
}

#[post("/add")]
async fn add(data: web::Json<PCManagerRequest>) -> web::Json<ResponseResult> {
    println!("{:#?}", data);
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Post PC Manager".to_string() })
}

#[get("/all")]
async fn all() -> web::Json<PcInformation> {
    web::Json(PcInformation {
        pcs:    Pcs {
            uuid: Uuid { hostname: "localhost1".to_string(), ip: "127.0.0.1".to_string() },
        },
        length: 1,
    })
}

// #[get("/specific")]
// async fn specific(data: actix_web::web::Query<SpecificRequest>) ->
// web::Json<PcInformation> {     dbg!(&data);
//     web::Json(PcInformation {
//         pcs:    Pcs {
//             uuid: Uuid { hostname: "localhost2".to_string(), ip:
// "127.0.0.1".to_string() },         },
//         length: 1,
//     })
// }

#[get("/specific")]
async fn specific(q: web::Json<SpecificRequest>) -> String {
    dbg!(&q);
    format!("got: {:?}", q.uuid)
}

async fn delete_pc(data: web::Json<DeletePcRequest>) -> web::Json<ResponseResult> {
    let _ = data;
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Delete PC".to_string() })
}

#[post("/reboot")]
async fn reboot(data: web::Json<UuidsRequest>) -> web::Json<ResponseResult> {
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Reboot PC".to_string() })
}

#[post("/shutdown")]
async fn shutdown(data: web::Json<UuidsRequest>) -> web::Json<ResponseResult> {
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Shutdown PC".to_string() })
}

async fn post_pcgroup(data: web::Json<PostPcgroupRequest>) -> web::Json<ResponseResult> {
    println!("{:#?}", data);
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Post PC Group".to_string() })
}

async fn get_pcgroup() -> web::Json<GetPcgroupResponseResult> {
    web::Json(GetPcgroupResponseResult {
        groups: types::Groups {
            vxlanid: types::Vxlanid {
                groupname: "group1".to_string(),
                pcs:       vec!["pc1".to_string(), "pc2".to_string()],
            },
        },
        length: 1,
    })
}

async fn put_pcgroup(data: web::Json<PutPcgroupRequest>) -> web::Json<ResponseResult> {
    println!("{:#?}", data);
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Put PC Group".to_string() })
}

async fn patch_pcgroup(data: web::Json<PatchPcgroupRequest>) -> web::Json<ResponseResult> {
    println!("{:#?}", data);
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Patch PC Group".to_string() })
}

async fn delete_pcgroup(data: web::Json<DeletePcGroupRequest>) -> web::Json<ResponseResult> {
    let _ = data;
    web::Json(ResponseResult { r#type: ResponseType::Ok, message: "Delete PC Group".to_string() })
}
