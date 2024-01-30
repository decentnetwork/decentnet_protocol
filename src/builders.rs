pub mod request {
    use std::collections::HashMap;

    use serde_bytes::ByteBuf;
    use serde_json::Value;

    use crate::templates::*;

    ///Peer requests
    pub fn get_file<'a>(
        site: &'a str,
        inner_path: &'a str,
        file_size: usize,
        location: usize,
        read_bytes: Option<usize>,
    ) -> (&'a str, GetFile) {
        (
            "getFile",
            GetFile {
                site: site.into(),
                inner_path: inner_path.into(),
                file_size,
                location,
                read_bytes,
            },
        )
    }

    pub fn stream_file<'a>(
        site: &'a str,
        inner_path: &'a str,
        file_size: usize,
        location: usize,
        read_bytes: usize,
    ) -> (&'a str, StreamFile) {
        (
            "streamFile",
            StreamFile {
                site: site.into(),
                inner_path: inner_path.into(),
                location,
                file_size,
                read_bytes,
            },
        )
    }

    pub fn pex<'a>(site: &'a str, need: usize) -> (&'a str, Pex) {
        (
            "pex",
            Pex {
                site: site.into(),
                peers: vec![],
                peers_onion: Some(vec![]),
                peers_ipv6: Some(vec![]),
                need,
            },
        )
    }

    pub fn update_site<'a>(
        site: &'a str,
        inner_path: &'a str,
        body: ByteBuf,
        diffs: HashMap<String, Vec<Value>>,
        modified: usize,
    ) -> (&'a str, Update) {
        (
            "update",
            Update {
                site: site.into(),
                inner_path: inner_path.into(),
                body,
                diffs,
                modified,
            },
        )
    }

    pub fn list_modified<'a>(site: &'a str, since: usize) -> (&'a str, ListModified) {
        ("listModified", ListModified { site: site.into(), since: since.into() })
    }

    pub fn get_hashfield<'a>(site: &'a str) -> (&'a str, GetHashfield) {
        ("getHashfield", GetHashfield { site: site.into() })
    }

    pub fn set_hashfield<'a>(site: &'a str, hashfield_raw: ByteBuf) -> (&'a str, SetHashfield) {
        (
            "setHashfield",
            SetHashfield {
                site: site.into(),
                hashfield_raw,
            },
        )
    }

    pub fn find_hash_ids<'a>(site: &'a str, hash_ids: Vec<usize>) -> (&'a str, FindHashIds) {
        ("findHashIds", FindHashIds { site: site.into(), hash_ids })
    }

    pub fn checkport<'a>(port: u16) -> (&'a str, Checkport) {
        ("checkport", Checkport { port })
    }

    ///Bigfile Plugin
    pub fn get_piece_fields<'a>(site: &'a str) -> (&'a str, GetPieceFields) {
        ("getPieceFields", GetPieceFields { site: site.into() })
    }

    pub fn set_piece_fields<'a>(
        site: &'a str,
        piecefields_packed: ByteBuf,
    ) -> (&'a str, SetPieceFields) {
        (
            "setPieceFields",
            SetPieceFields {
                site: site.into(),
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

    pub fn update_site(ok: &str) -> UpdateSiteResponse {
        UpdateSiteResponse { ok: ok.into()}
    }

    pub fn list_modified(modified_files: HashMap<String, usize>) -> ListModifiedResponse {
        ListModifiedResponse { modified_files }
    }

    pub fn get_hashfield(hashfield_raw: ByteBuf) -> GetHashfieldResponse {
        GetHashfieldResponse { hashfield_raw }
    }

    pub fn set_hashfield(ok: &str) -> SetHashfieldResponse {
        SetHashfieldResponse { ok: ok.into() }
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

    pub fn checkport(status: &str, ip_external: &str) -> CheckportResponse {
        CheckportResponse {
            status: status.into(),
            ip_external: ip_external.into(),
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
