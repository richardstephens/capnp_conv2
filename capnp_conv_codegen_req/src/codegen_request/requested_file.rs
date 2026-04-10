use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

#[capnp_conv(schema_capnp::code_generator_request::requested_file)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedFile {
    /// ID of the file.
    pub id: u64,
    /// Name of the file as it appeared on the command-line (minus the src-prefix). You may use
    /// this to decide where to write the output.
    pub filename: String,
    /// List of all imported paths seen in this file.
    pub imports: Vec<RequestedFileImport>,
}

#[capnp_conv(schema_capnp::code_generator_request::requested_file::import)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestedFileImport {
    /// ID of the imported file.
    pub id: u64,
    /// Name which *this* file used to refer to the foreign file. This may be a relative name.
    /// This information is provided because it might be useful for code generation, e.g. to
    /// generate `#include` directives in C++.
    pub name: String,
}
