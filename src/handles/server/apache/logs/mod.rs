use serde::Serialize;
use crate::handles::server::apache::logs::access_log::Access_log;
use crate::commons::error_logs::Error_log;

pub mod access_log;


#[derive(Debug, Serialize)]
pub struct Logs{
    #[serde(rename = "Error_log")]
    pub error_log: Vec<Error_log>,
    #[serde(rename = "Errlength")]
    pub errlength: usize,
    #[serde(rename = "Access_log")]
    pub access_log: Vec<Access_log>,
    #[serde(rename = "Acclength")]
    pub acclength: usize,
}