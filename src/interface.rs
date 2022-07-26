use std::collections::HashMap;

use serde_bytes::ByteBuf;

use crate::templates::*;

#[async_trait::async_trait]
pub trait RequestImpl {
    type Error;

    async fn handshake(&mut self) -> Result<Handshake, Self::Error>;

    async fn ping(&mut self) -> Result<bool, Self::Error>;

    async fn get_file(
        &mut self,
        site: String,
        inner_path: String,
        file_size: usize,
        location: usize,
        read_bytes: Option<usize>,
    ) -> Result<GetFileResponse, Self::Error>;

    async fn stream_file(
        &mut self,
        site: String,
        inner_path: String,
    ) -> Result<StreamFileResponse, Self::Error>;

    async fn list_modified(
        &mut self,
        site: String,
        since: usize,
    ) -> Result<ListModifiedResponse, Self::Error>;

    async fn pex(&mut self, site: String) -> Result<PexResponse, Self::Error>;

    async fn update(
        &mut self,
        site: String,
        inner_path: String,
        body: ByteBuf,
        diffs: HashMap<String, Vec<serde_json::Value>>,
        modified: usize,
    ) -> Result<UpdateSiteResponse, Self::Error>;
}

#[async_trait::async_trait]
pub trait ResponseImpl {
    type Error;

    async fn handshake(&mut self, id: usize) -> Result<bool, Self::Error>;

    async fn ping(&mut self, id: usize) -> Result<bool, Self::Error>;

    async fn get_file(
        &mut self,
        id: usize,
        site: ByteBuf,
        size: usize,
        location: usize,
    ) -> Result<bool, Self::Error>;

    async fn stream_file(
        &mut self,
        id: usize,
        stream_bytes: usize,
        location: usize,
        size: usize,
        bytes: ByteBuf,
    ) -> Result<bool, Self::Error>;

    async fn list_modified(
        &mut self,
        id: usize,
        modified_files: HashMap<String, usize>,
    ) -> Result<bool, Self::Error>;

    async fn pex(
        &mut self,
        id: usize,
        peers: Vec<ByteBuf>,
        peers_ipv6: Vec<ByteBuf>,
        peers_onion: Vec<ByteBuf>,
    ) -> Result<bool, Self::Error>;

    async fn update(&mut self, id: usize, msg: &str) -> Result<bool, Self::Error>;
}
