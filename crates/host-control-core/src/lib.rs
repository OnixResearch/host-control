//! Transport-neutral host-control protocol functional core.
//!
//! Consumer shells retain sockets, PTYs, processes, persistence, clocks,
//! rendering, policy, native identifiers, and product state.
//!
// r[impl host_control.core.purity]
// r[impl host_control.core.non_claims]

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

mod hostcontrol;
#[cfg(feature = "preserves")]
pub mod preserves_wire;
pub mod profiles;
/// Pure resource-watermark decisions over explicit observations.
pub mod shedding;
/// Shared bounded field validation and contract diagnostics.
pub mod validation;
#[cfg(feature = "serde")]
pub mod wire;

pub use hostcontrol::*;
pub use validation::AdapterContractError;
