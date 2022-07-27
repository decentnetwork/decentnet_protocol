pub mod request {
    use std::collections::HashMap;

    use serde_bytes::ByteBuf;
    use serde_json::Value;

    use crate::templates::*;

    ///Peer requests
    pub fn get_file<'a>(
        site: String,
        inner_path: String,
        file_size: usize,
        location: usize,
        read_bytes: Option<usize>,
    ) -> (&'a str, GetFile) {
        (
            "getFile",
            GetFile {
                site,
                inner_path,
                file_size,
                location,
                read_bytes,
            },
        )
    }

    pub fn stream_file<'a>(
        site: String,
        inner_path: String,
        location: usize,
        file_size: usize,
        read_bytes: usize,
    ) -> (&'a str, StreamFile) {
        (
            "streamFile",
            StreamFile {
                site,
                inner_path,
                location,
                file_size,
                read_bytes,
            },
        )
    }

    pub fn pex<'a>(site: String, need: usize) -> (&'a str, Pex) {
        (
            "pex",
            Pex {
                site,
                peers: vec![],
                peers_onion: Some(vec![]),
                peers_ipv6: Some(vec![]),
                need,
            },
        )
    }

    pub fn update_site<'a>(
        site: String,
        inner_path: String,
        body: ByteBuf,
        diffs: HashMap<String, Vec<Value>>,
        modified: usize,
    ) -> (&'a str, Update) {
        (
            "update",
            Update {
                site,
                inner_path,
                body,
                diffs,
                modified,
            },
        )
    }

    pub fn list_modified<'a>(site: String, since: usize) -> (&'a str, ListModified) {
        ("listModified", ListModified { site, since })
    }

    pub fn get_hashfield<'a>(site: String) -> (&'a str, GetHashfield) {
        ("getHashfield", GetHashfield { site })
    }

    pub fn set_hashfield<'a>(site: String, hashfield_raw: ByteBuf) -> (&'a str, SetHashfield) {
        (
            "setHashfield",
            SetHashfield {
                site,
                hashfield_raw,
            },
        )
    }

    pub fn find_hash_ids<'a>(site: String, hash_ids: Vec<usize>) -> (&'a str, FindHashIds) {
        ("findHashIds", FindHashIds { site, hash_ids })
    }

    pub fn checkport<'a>(port: u16) -> (&'a str, Checkport) {
        ("checkport", Checkport { port })
    }

    ///Bigfile Plugin
    pub fn get_piece_fields<'a>(site: String) -> (&'a str, GetPieceFields) {
        ("getPieceFields", GetPieceFields { site })
    }

    pub fn set_piece_fields<'a>(
        site: String,
        piecefields_packed: ByteBuf,
    ) -> (&'a str, SetPieceFields) {
        (
            "setPieceFields",
            SetPieceFields {
                site,
                piecefields_packed,
            },
        )
    }
}

pub mod response {
    use std::collections::HashMap;

    use serde_bytes::ByteBuf;

    use crate::templates::*;

    ///Peer requests
    pub fn get_file(body: ByteBuf, size: usize, location: usize) -> GetFileResponse {
        GetFileResponse {
            body,
            size,
            location,
        }
    }

    pub fn stream_file(stream_bytes: usize, location: usize, size: usize) -> StreamFileResponse {
        StreamFileResponse {
            stream_bytes,
            location,
            size,
        }
    }

    pub fn pex(
        peers: Vec<ByteBuf>,
        peers_ipv6: Vec<ByteBuf>,
        peers_onion: Vec<ByteBuf>,
    ) -> PexResponse {
        PexResponse {
            peers,
            peers_ipv6,
            peers_onion,
        }
    }

    pub fn update_site(ok: String) -> UpdateSiteResponse {
        UpdateSiteResponse { ok }
    }

    pub fn list_modified(modified_files: HashMap<String, usize>) -> ListModifiedResponse {
        ListModifiedResponse { modified_files }
    }

    pub fn get_hashfield(hashfield_raw: ByteBuf) -> GetHashfieldResponse {
        GetHashfieldResponse { hashfield_raw }
    }

    pub fn set_hashfield(ok: String) -> SetHashfieldResponse {
        SetHashfieldResponse { ok }
    }

    pub fn find_hash_ids(
        peers: HashMap<usize, Vec<ByteBuf>>,
        peers_onion: HashMap<usize, Vec<ByteBuf>>,
        peers_ipv6: HashMap<usize, Vec<ByteBuf>>,
        my: Vec<usize>,
    ) -> FindHashIdsResponse {
        FindHashIdsResponse {
            peers,
            peers_onion,
            peers_ipv6,
            my,
        }
    }

    pub fn checkport(status: String, ip_external: String) -> CheckportResponse {
        CheckportResponse {
            status,
            ip_external,
        }
    }

    ///Bigfile Plugin
    pub fn get_piece_fields(piecefields_packed: ByteBuf) -> GetPieceFieldsResponse {
        GetPieceFieldsResponse { piecefields_packed }
    }

    pub fn set_piece_fields(ok: bool) -> SetPieceFieldsResponse {
        SetPieceFieldsResponse { ok }
    }
}
