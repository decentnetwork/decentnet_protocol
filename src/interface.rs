use std::collections::HashMap;
use std::hash::Hash;

use serde_bytes::ByteBuf;
use serde_json::Value;

use crate::templates::*;
use crate::utils::Either;

#[async_trait::async_trait]
pub trait RequestImpl {
    type Error;

    async fn handshake(&mut self) -> Result<Handshake, Self::Error>;

    async fn ping(&mut self) -> Result<bool, Self::Error>;

    async fn get_file(
        &mut self,
        site: &str,
        inner_path: &str,
        file_size: usize,
        location: usize,
        read_bytes: Option<usize>,
    ) -> Result<Either<GetFileResponse, ErrorResponse>, Self::Error>;

    async fn stream_file(
        &mut self,
        site: &str,
        inner_path: &str,
    ) -> Result<Either<StreamFileResponse, ErrorResponse>, Self::Error>;

    async fn list_modified(
        &mut self,
        site: &str,
        since: usize,
    ) -> Result<ListModifiedResponse, Self::Error>;

    async fn pex(&mut self, site: &str) -> Result<PexResponse, Self::Error>;

    async fn update(
        &mut self,
        site: &str,
        inner_path: &str,
        body: ByteBuf,
        diffs: HashMap<String, Vec<Value>>,
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

pub trait Requestable {
    type Key: PartialEq + Eq + Hash + Send + Clone;

    /// Returns the request's ID if it has one.
    fn req_id(&self) -> Option<Self::Key>;
    /// Returns the ID of the request responded to.
    fn to(&self) -> Option<Self::Key>;

    /// If the message has a request ID, it is a request.
    /// It is possible for a response to simultaneously be
    /// a request.
    fn is_request(&self) -> bool {
        match self.req_id() {
            Some(_) => true,
            None => false,
        }
    }

    /// If the message has a response ID, it is a response.
    /// It is possible for a response to simultaneously be
    /// a request.
    fn is_response(&self) -> bool {
        match self.to() {
            Some(_) => true,
            None => false,
        }
    }
}
