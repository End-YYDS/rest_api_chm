use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LoadStatus {
    Load,
    Notload,
    Installed,
    Notinstall,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EnableStatus {
    Enable,
    Disable,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub name:         String,
    pub version:      String,
    pub description:  String,
    pub author:       String,
    pub loadstatus:   LoadStatus,
    pub enablestatus: EnableStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModuleCollection {
    pub modules: HashMap<String, Module>, // <-- 動態 key
    pub length:  usize,
}
