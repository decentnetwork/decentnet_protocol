use std::collections::HashMap;

use crate::utils::is_default;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OkResponse {
    pub ok: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
#[serde(default)]
pub struct Handshake {
    pub peer_id: String,
    pub fileserver_port: usize,
    pub time: u64,
    #[serde(default, skip_serializing_if = "is_default")]
    pub crypt: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub crypt_supported: Vec<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub use_bin_type: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub onion: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub protocol: String,
    #[serde(default, skip_serializing_if = "is_default")]
    pub port_opened: Option<bool>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub rev: usize,
    #[serde(default, skip_serializing_if = "is_default", rename = "target_ip")]
    pub target_address: Option<String>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ping();

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PingResponse {
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetFile {
    pub site: String,
    pub inner_path: String,
    pub location: usize,
    #[serde(skip_serializing_if = "is_default")]
    pub read_bytes: Option<usize>,
    #[serde(skip_serializing_if = "is_default")]
    pub file_size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetFileResponse {
    pub body: ByteBuf,
    pub location: usize,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamFile {
    pub site: String,
    pub inner_path: String,
    pub location: usize,
    #[serde(skip_serializing_if = "is_default")]
    pub read_bytes: usize,
    #[serde(skip_serializing_if = "is_default")]
    pub file_size: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct StreamFileResponse {
    pub location: usize,
    pub size: usize,
    pub stream_bytes: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Pex {
    pub site: String,
    pub peers: Vec<ByteBuf>,
    #[serde(skip_serializing_if = "is_default")]
    pub peers_onion: Option<Vec<ByteBuf>>,
    #[serde(skip_serializing_if = "is_default")]
    pub peers_ipv6: Option<Vec<ByteBuf>>,
    pub need: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PexResponse {
    pub peers: Vec<ByteBuf>,
    pub peers_ipv6: Vec<ByteBuf>,
    pub peers_onion: Vec<ByteBuf>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Update {
    pub site: String,
    pub inner_path: String,
    pub body: ByteBuf,
    pub modified: usize,
    pub diffs: HashMap<String, Vec<Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateSiteResponse {
    pub ok: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListModified {
    pub site: String,
    pub since: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ListModifiedResponse {
    pub modified_files: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetHashfield {
    pub site: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetHashfieldResponse {
    pub hashfield_raw: ByteBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetHashfield {
    pub site: String,
    pub hashfield_raw: ByteBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetHashfieldResponse {
    pub ok: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FindHashIds {
    pub site: String,
    pub hash_ids: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct FindHashIdsResponse {
    pub peers: HashMap<usize, Vec<ByteBuf>>,
    pub peers_ipv6: HashMap<usize, Vec<ByteBuf>>,
    pub peers_onion: HashMap<usize, Vec<ByteBuf>>,
    pub my: Vec<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Checkport {
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct CheckportResponse {
    pub status: String,
    pub ip_external: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPieceFields {
    pub site: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct GetPieceFieldsResponse {
    pub piecefields_packed: ByteBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPieceFields {
    pub site: String,
    pub piecefields_packed: ByteBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SetPieceFieldsResponse {
    pub ok: bool,
}
