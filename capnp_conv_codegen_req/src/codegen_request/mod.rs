use crate::codegen_request::misc::CapnpVersion;
use crate::codegen_request::node::Node;
use crate::codegen_request::requested_file::RequestedFile;
use crate::codegen_request::source_info::SourceInfo;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

pub mod misc;
pub mod node;
pub mod requested_file;
pub mod source_info;

#[capnp_conv(schema_capnp::code_generator_request)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeGeneratorRequest {
    /// All nodes parsed by the compiler, including for the files on the command line and their
    /// imports.
    pub nodes: Vec<Node>,
    /// Files which were listed on the command line.
    pub requested_files: Vec<RequestedFile>,
    /// Version of the `capnp` executable. Generally, code generators should ignore this, but the
    /// code generators that ship with `capnp` itself will print a warning if this mismatches since
    /// that probably indicates something is misconfigured.
    ///
    /// The first version of `capnp` to set this was 0.6.0. So, if it's missing, the compiler
    /// version is older than that.
    pub capnp_version: CapnpVersion,
    /// Information about the original source code for each node, where available. This array may be
    /// omitted or may be missing some nodes if no info is available for them.
    pub source_info: Vec<SourceInfo>,
}
