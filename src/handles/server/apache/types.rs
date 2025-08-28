use serde::Serialize;
use crate::commons::{CommonInfo, Status};
use crate::handles::server::apache::logs::Logs;

#[derive(Debug, Serialize)]
pub struct ApacheResponse{
    #[serde(flatten)]
    pub common_info: CommonInfo,
    #[serde(rename = "Connections")]
    pub connections: i64,
    #[serde(rename = "Logs")]
    pub logs: Logs,
}