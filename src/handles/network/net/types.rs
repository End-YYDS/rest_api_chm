use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum NicType {
    #[serde(rename = "Virtual")]
    Virtual,
    #[serde(rename = "Physical")]
    Physical,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum NicStatus {
    #[serde(rename = "Up")]
    Up,
    #[serde(rename = "Down")]
    Down,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NetworkItem {
    // #[serde(rename = "Name")] // 規格目前註解掉
    // pub name: String,
    #[serde(rename = "Type")]
    pub nic_type:  NicType,
    #[serde(rename = "Ipv4")]
    pub ipv4:      String,
    #[serde(rename = "Netmask")]
    pub netmask:   String,
    #[serde(rename = "Mac")]
    pub mac:       String,
    #[serde(rename = "Broadcast")]
    pub broadcast: String,
    #[serde(rename = "Mtu")]
    pub mtu:       i64,
    #[serde(rename = "Status")]
    pub status:    NicStatus,
}

#[derive(Debug, Serialize)]
pub struct PcNetworks {
    #[serde(rename = "Networks")]
    pub networks: HashMap<String, NetworkItem>, // nid -> item
    #[serde(rename = "Length")]
    pub length:   usize,
}

#[derive(Debug, Serialize)]
pub struct GetAllNetResponse {
    #[serde(rename = "Pcs")]
    pub pcs:    HashMap<String, PcNetworks>, // uuid -> networks
    #[serde(rename = "Length")]
    pub length: usize,
}

// POST /api/network/net
#[derive(Debug, Deserialize)]
pub struct CreateNetRequest {
    #[serde(rename = "Nid")]
    pub nid:       String,
    #[serde(rename = "Type")]
    pub nic_type:  NicType,
    #[serde(rename = "Ipv4")]
    pub ipv4:      String,
    #[serde(rename = "Netmask")]
    pub netmask:   String,
    #[serde(rename = "Mac")]
    pub mac:       String,
    #[serde(rename = "Broadcast")]
    pub broadcast: String,
    #[serde(rename = "Mtu")]
    pub mtu:       i64,
    #[serde(rename = "Status")]
    pub status:    NicStatus,
}

// DELETE /api/network/net
#[derive(Debug, Deserialize)]
pub struct DeleteNetRequest {
    #[serde(rename = "Nid")]
    pub nid: String,
}

// PATCH /api/network/net —— 單一項目
#[derive(Debug, Deserialize)]
pub enum PatchField {
    #[serde(rename = "Type")]
    Type,
    #[serde(rename = "Ipv4")]
    Ipv4,
    #[serde(rename = "Netmask")]
    Netmask,
    #[serde(rename = "Mac")]
    Mac,
    #[serde(rename = "Broadcast")]
    Broadcast,
    #[serde(rename = "Mtu")]
    Mtu,
    #[serde(rename = "Status")]
    Status,
}

#[derive(Debug, Deserialize)]
pub struct PatchNetRequest {
    #[serde(rename = "Nid")]
    pub nid:       String, // ← 實務上需要指定哪一筆
    #[serde(rename = "Type")]
    pub field:     PatchField,
    // 可能值（僅需提供對應欄位）
    #[serde(rename = "Ipv4")]
    pub ipv4:      Option<String>,
    #[serde(rename = "Netmask")]
    pub netmask:   Option<String>,
    #[serde(rename = "Mac")]
    pub mac:       Option<String>,
    #[serde(rename = "Broadcast")]
    pub broadcast: Option<String>,
    #[serde(rename = "Mtu")]
    pub mtu:       Option<i64>,
    #[serde(rename = "Status")]
    pub status:    Option<NicStatus>,
    #[serde(rename = "TypeValue")]
    pub nic_type:  Option<NicType>, // 若 field = Type
}

// PUT /api/network/net —— 整筆更新
#[derive(Debug, Deserialize)]
pub struct PutNetRequest {
    #[serde(rename = "Nid")]
    pub nid:       String, // ← 實務上需要指定哪一筆
    #[serde(rename = "Type")]
    pub nic_type:  NicType,
    #[serde(rename = "Ipv4")]
    pub ipv4:      String,
    #[serde(rename = "Netmask")]
    pub netmask:   String,
    #[serde(rename = "Mac")]
    pub mac:       String,
    #[serde(rename = "Broadcast")]
    pub broadcast: String,
    #[serde(rename = "Mtu")]
    pub mtu:       i64,
    #[serde(rename = "Status")]
    pub status:    NicStatus,
}

// action up/down
#[derive(Debug, Deserialize)]
pub struct ActionNetRequest {
    #[serde(rename = "Nid")]
    pub nid: String,
}
