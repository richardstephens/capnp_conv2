use crate::codegen_request::node::types::Type;
use capnp::schema_capnp;
use capnp_conv2::capnp_conv;
use serde::{Deserialize, Serialize};

/// Specifies bindings for parameters of generics. Since these bindings turn a generic into a
/// non-generic, we call it the "brand".
#[capnp_conv(schema_capnp::brand)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brand {
    /// For each of the target type and each of its parent scopes, a parameterization may be
    /// included in this list. If no parameterization is included for a particular relevant scope,
    /// then either that scope has no parameters or all parameters should be considered to be
    /// `AnyPointer`.
    pub scopes: Vec<BrandScope>,
}

#[capnp_conv(schema_capnp::brand::scope)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandScope {
    /// ID of the scope to which these params apply.
    pub scope_id: u64,
    #[capnp_conv(type = "unnamed_union")]
    pub kind: BrandScopeKind,
}

#[capnp_conv(schema_capnp::brand::scope)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrandScopeKind {
    /// List of parameter bindings.
    Bind(Vec<BrandBinding>),
    /// The place where the Brand appears is within this scope or a sub-scope, and bindings
    /// for this scope are deferred to later Brand applications. This is equivalent to a
    /// pass-through binding list, where each of this scope's parameters is bound to itself.
    Inherit(()),
}

#[capnp_conv(schema_capnp::brand::binding)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrandBinding {
    Unbound(()),
    Type(Type),
}
