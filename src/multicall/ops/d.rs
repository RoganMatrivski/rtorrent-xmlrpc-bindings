//! Rtorrent d.* multicall operations

use crate::{multicall::raw, Server};
use std::borrow::Cow;
use std::marker::PhantomData;

super::op_type! {
    /// A `d.*` operation for multicalls.
    DownloadMultiCallOp
}

/// The `MultiBuilder` type is a tool for building queries of one or more fields across many
/// downloads in a single XMLRPC call.  The query results are nicely typed.
///
/// ## Usage
///
/// Example: Print name, size, and upload ratio for all downloads in the "default" view.
///
/// ```no_run
/// use rtorrent_xmlrpc_bindings as rtorrent;
/// use rtorrent::multicall::d;
///
/// let my_handle = rtorrent::Server::new("http://1.2.3.4/RPC2");
///
/// d::MultiBuilder::new(&my_handle, "default")
///     .call(d::NAME)
///     .call(d::RATIO)
///     .call(d::SIZE_BYTES)
///     .invoke()?
///     .iter()
///     .for_each(|(name, ratio, bytes)| {
///         println!("{}: {} bytes, {} ratio", name, bytes, ratio);
///     });
/// # Ok::<(), rtorrent::Error>(())
/// ```
///
/// The `call()` method can be invoked repeatedly to add more columns to the query -- in the above
/// example, selecting the `NAME`, `RATIO`, and `SIZE_BYTES` columns.
pub struct MultiBuilder {
    pub(crate) inner: raw::MultiBuilder,
}

impl MultiBuilder {
    /// Start building a multicall over downloads in some specific `view` on `server`.
    pub fn new(server: &Server, view: &str) -> Self {
        Self {
            inner: raw::MultiBuilder::new(server, "d.multicall2", "", view),
        }
    }
}

macro_rules! define_builder {
    ( $prev: ident, $name: ident, $($phantoms:ident $ty:ident),* | $phantom_last:ident $ty_last:ident ) => {
        ops::define_builder!(DownloadMultiCallOp, $prev, $name, $($phantoms $ty),* | $phantom_last $ty_last);
    }
}
pub(crate) use define_builder;

macro_rules! d_op_const {
    ( $(#[$meta:meta])* $name: ident, $res: ty, $api: literal ) => {
        super::op_const!( $(#[$meta])* DownloadMultiCallOp, $name, $res, "d.", $api);
    };
}

d_op_const!(
    /// Infohash for this torrent.
    HASH, String, "hash");
d_op_const!(
    /// Is this torrent active?
    IS_ACTIVE, bool, "is_active");
d_op_const!(
    /// Unstructured error messages, either generated by rtorrent, or forwarded from the
    /// tracker.
    MESSAGE, String, "message");
d_op_const!(
    /// Get the name of the torrent.
    NAME, String, "name");
d_op_const!(
    /// Get the upload/download ratio for this download.
    RATIO, f64, "ratio");
d_op_const!(
    /// Get the size, in bytes, of the torrent contents.
    SIZE_BYTES, i64, "size_bytes");