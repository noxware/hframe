//! This module contains different strategies that can be used when masking the
//! HTML content. By default, the `Auto` strategy is used which delegates the
//! work to some other strategy depending on the browser being use.

mod auto;
mod data_mask;
mod document_mask;
mod nop;

pub(crate) use auto::Auto;
pub(crate) use data_mask::DataMask;
pub(crate) use document_mask::DocumentMask;
pub(crate) use nop::Nop;
