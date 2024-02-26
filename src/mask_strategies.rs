//! This module contains different strategies that can be used when masking the
//! HTML content. By default, the `Auto` strategy is used which delegates the
//! work to some other strategy depending on the browser being use.

mod auto;
mod data_mask;
mod document_mask;
mod hide;
mod nop;

pub use auto::Auto;
pub use data_mask::DataMask;
pub use document_mask::DocumentMask;
pub use hide::Hide;
pub use nop::Nop;
