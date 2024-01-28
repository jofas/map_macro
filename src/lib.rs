#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "hashbrown")]
pub mod hashbrown;

#[cfg(feature = "std")]
mod _std;
